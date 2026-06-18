import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';

@Component({
  selector: 'app-auth',
  standalone: true,
  imports: [CommonModule, FormsModule],
  template: `
    <div class="auth">
      <header class="page-header">
        <h1>Authentication</h1>
        <p class="subtitle">Enter your master passphrase to start a session</p>
      </header>

      <div class="auth-card">
        <div class="field">
          <label for="passphrase">Master Passphrase</label>
          <input
            id="passphrase"
            type="password"
            [(ngModel)]="passphrase"
            placeholder="Enter passphrase"
            (keyup.enter)="authenticate()"
            autocomplete="current-password"
          />
        </div>

        <div class="field">
          <label for="duration">Session Duration</label>
          <select id="duration" [(ngModel)]="duration">
            <option value="1h">1 hour</option>
            <option value="2h">2 hours</option>
            <option value="4h">4 hours</option>
            <option value="8h">8 hours</option>
            <option value="24h">24 hours</option>
          </select>
        </div>

        @if (error) {
          <div class="error">{{ error }}</div>
        }

        @if (success) {
          <div class="success">{{ success }}</div>
        }

        <button class="btn-primary" (click)="authenticate()" [disabled]="!passphrase">
          Authenticate
        </button>

        <p class="hint">
          Run <code>envsec auth</code> in your terminal to authenticate the CLI.
        </p>
      </div>
    </div>
  `,
  styles: [`
    .page-header {
      margin-bottom: 2rem;
    }
    .page-header h1 {
      font-size: 1.75rem;
      font-weight: 700;
    }
    .subtitle {
      color: var(--text-muted);
      margin-top: 0.25rem;
    }
    .auth-card {
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: var(--radius);
      padding: 2rem;
      max-width: 480px;
    }
    .field {
      margin-bottom: 1.5rem;
    }
    label {
      display: block;
      font-size: 0.85rem;
      font-weight: 600;
      margin-bottom: 0.5rem;
      color: var(--text-muted);
    }
    input, select {
      width: 100%;
      padding: 0.65rem 0.75rem;
      background: var(--bg);
      border: 1px solid var(--border);
      border-radius: var(--radius-sm);
      color: var(--text);
      transition: border-color var(--transition);
    }
    input:focus, select:focus {
      outline: none;
      border-color: var(--accent);
    }
    .btn-primary {
      width: 100%;
      padding: 0.75rem;
      background: var(--accent);
      color: white;
      border: none;
      border-radius: var(--radius-sm);
      font-weight: 600;
      cursor: pointer;
      transition: background var(--transition);
    }
    .btn-primary:hover:not(:disabled) {
      background: var(--accent-hover);
    }
    .btn-primary:disabled {
      opacity: 0.5;
      cursor: not-allowed;
    }
    .error {
      background: #fef2f2;
      color: var(--danger);
      padding: 0.75rem;
      border-radius: var(--radius-sm);
      margin-bottom: 1rem;
      font-size: 0.9rem;
    }
    .success {
      background: #f0fdf4;
      color: var(--success);
      padding: 0.75rem;
      border-radius: var(--radius-sm);
      margin-bottom: 1rem;
      font-size: 0.9rem;
    }
    .hint {
      margin-top: 1.5rem;
      color: var(--text-dim);
      font-size: 0.85rem;
    }
    .hint code {
      font-weight: 600;
    }
  `]
})
export class AuthComponent {
  passphrase = '';
  duration = '2h';
  error = '';
  success = '';

  constructor(private router: Router) {}

  authenticate(): void {
    this.error = '';
    this.success = '';

    if (!this.passphrase) {
      this.error = 'Passphrase is required';
      return;
    }

    this.success = 'Session started. You can now use envsec commands.';
    this.passphrase = '';
  }
}
