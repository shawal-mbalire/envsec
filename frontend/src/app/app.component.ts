import { Component } from '@angular/core';
import { RouterOutlet, RouterLink, RouterLinkActive } from '@angular/router';
import { ThemeService } from './services/theme.service';
import { AsyncPipe } from '@angular/common';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [RouterOutlet, RouterLink, RouterLinkActive, AsyncPipe],
  template: `
    <nav class="nav">
      <div class="nav-inner container">
        <a routerLink="/" class="nav-brand">
          <span class="nav-brand-icon">#</span>
          envsec
        </a>
        <div class="nav-links">
          <a routerLink="/" routerLinkActive="active" [routerLinkActiveOptions]="{exact: true}">Dashboard</a>
          <a routerLink="/projects" routerLinkActive="active">Projects</a>
          <a routerLink="/auth" routerLinkActive="active">Auth</a>
        </div>
        <button class="theme-toggle" (click)="toggleTheme()" [attr.aria-label]="'Toggle theme'">
          {{ (themeService.current$ | async) === 'dark' ? 'Light' : 'Dark' }}
        </button>
      </div>
    </nav>
    <main class="main container">
      <router-outlet></router-outlet>
    </main>
  `,
  styles: [`
    .nav {
      border-bottom: 1px solid var(--border);
      background: var(--bg);
      position: sticky;
      top: 0;
      z-index: 100;
    }
    .nav-inner {
      display: flex;
      align-items: center;
      height: 56px;
      gap: 2rem;
    }
    .nav-brand {
      font-weight: 700;
      font-size: 1.1rem;
      color: var(--text);
      display: flex;
      align-items: center;
      gap: 0.5rem;
    }
    .nav-brand:hover {
      color: var(--accent);
    }
    .nav-brand-icon {
      background: var(--accent);
      color: white;
      width: 28px;
      height: 28px;
      display: flex;
      align-items: center;
      justify-content: center;
      border-radius: var(--radius-sm);
      font-size: 0.85rem;
      font-weight: 700;
    }
    .nav-links {
      display: flex;
      gap: 0.25rem;
      flex: 1;
    }
    .nav-links a {
      color: var(--text-muted);
      padding: 0.5rem 0.75rem;
      border-radius: var(--radius-sm);
      font-size: 0.9rem;
      transition: all var(--transition);
    }
    .nav-links a:hover {
      color: var(--text);
      background: var(--surface);
    }
    .nav-links a.active {
      color: var(--accent);
      background: var(--accent-light);
    }
    .theme-toggle {
      background: var(--surface);
      border: 1px solid var(--border);
      color: var(--text-muted);
      padding: 0.4rem 0.75rem;
      border-radius: var(--radius-sm);
      cursor: pointer;
      font-size: 0.8rem;
      transition: all var(--transition);
    }
    .theme-toggle:hover {
      background: var(--surface-hover);
      color: var(--text);
    }
    .main {
      padding-top: 2rem;
      padding-bottom: 4rem;
    }
  `]
})
export class AppComponent {
  constructor(public themeService: ThemeService) {}

  toggleTheme(): void {
    this.themeService.toggle();
  }
}
