import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterLink } from '@angular/router';
import { ApiService } from '../../services/api.service';
import { Project } from '../../types';

@Component({
  selector: 'app-dashboard',
  standalone: true,
  imports: [CommonModule, RouterLink],
  template: `
    <div class="dashboard">
      <header class="page-header">
        <h1>Dashboard</h1>
        <p class="subtitle">Local-first encrypted secret manager</p>
      </header>

      <section class="status-grid">
        <div class="card">
          <div class="card-label">Session</div>
          <div class="card-value" [class.status-ok]="status?.session?.includes('active')" [class.status-err]="!status?.session?.includes('active')">
            {{ status?.session || 'checking...' }}
          </div>
        </div>
        <div class="card">
          <div class="card-label">Active Project</div>
          <div class="card-value">{{ status?.active_project || 'none' }}</div>
        </div>
        <div class="card">
          <div class="card-label">Session Duration</div>
          <div class="card-value">{{ status?.session_duration || '--' }}</div>
        </div>
        <div class="card">
          <div class="card-label">Clipboard Clear</div>
          <div class="card-value">{{ status?.clipboard_clear || '--' }}</div>
        </div>
      </section>

      <section class="projects-section">
        <div class="section-header">
          <h2>Projects</h2>
          <a routerLink="/projects" class="link-btn">View all</a>
        </div>

        @if (loading) {
          <div class="empty">Loading...</div>
        } @else if (projects.length === 0) {
          <div class="empty">
            <p>No projects yet.</p>
            <code>envsec use &lt;project&gt; &lt;environment&gt;</code>
          </div>
        } @else {
          <div class="project-list">
            @for (project of projects; track project.name) {
              <div class="project-card">
                <div class="project-name">{{ project.name }}</div>
                <div class="project-envs">
                  @for (env of getEnvs(project); track env) {
                    <span class="tag">{{ env }}</span>
                  }
                </div>
                <div class="project-count">
                  {{ getKeyCount(project) }} secrets
                </div>
              </div>
            }
          </div>
        }
      </section>

      <section class="quick-start">
        <h2>Quick Start</h2>
        <div class="commands">
          <div class="command">
            <code>envsec init</code>
            <span>Create vault and set passphrase</span>
          </div>
          <div class="command">
            <code>envsec use myapp dev</code>
            <span>Bind project</span>
          </div>
          <div class="command">
            <code>envsec set KEY value</code>
            <span>Set a secret</span>
          </div>
          <div class="command">
            <code>envsec get KEY</code>
            <span>Copy to clipboard</span>
          </div>
          <div class="command">
            <code>envsec run -- cmd</code>
            <span>Run with secrets</span>
          </div>
        </div>
      </section>
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
    .status-grid {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
      gap: 1rem;
      margin-bottom: 2.5rem;
    }
    .card {
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: var(--radius);
      padding: 1.25rem;
    }
    .card-label {
      font-size: 0.8rem;
      color: var(--text-muted);
      text-transform: uppercase;
      letter-spacing: 0.05em;
      margin-bottom: 0.5rem;
    }
    .card-value {
      font-size: 1rem;
      font-weight: 600;
    }
    .status-ok { color: var(--success); }
    .status-err { color: var(--danger); }
    .section-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      margin-bottom: 1rem;
    }
    .section-header h2 {
      font-size: 1.25rem;
      font-weight: 600;
    }
    .link-btn {
      font-size: 0.85rem;
      color: var(--accent);
    }
    .project-list {
      display: grid;
      gap: 0.75rem;
      margin-bottom: 2.5rem;
    }
    .project-card {
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: var(--radius);
      padding: 1rem 1.25rem;
      display: flex;
      align-items: center;
      gap: 1rem;
      transition: border-color var(--transition);
    }
    .project-card:hover {
      border-color: var(--accent);
    }
    .project-name {
      font-weight: 600;
      min-width: 120px;
    }
    .project-envs {
      display: flex;
      gap: 0.5rem;
      flex: 1;
    }
    .tag {
      background: var(--accent-light);
      color: var(--accent);
      padding: 0.15rem 0.5rem;
      border-radius: var(--radius-sm);
      font-size: 0.8rem;
      font-weight: 500;
    }
    .project-count {
      color: var(--text-muted);
      font-size: 0.85rem;
    }
    .empty {
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: var(--radius);
      padding: 2rem;
      text-align: center;
      color: var(--text-muted);
      margin-bottom: 2.5rem;
    }
    .empty code {
      display: inline-block;
      margin-top: 0.5rem;
    }
    .quick-start h2 {
      font-size: 1.25rem;
      font-weight: 600;
      margin-bottom: 1rem;
    }
    .commands {
      display: grid;
      gap: 0.5rem;
    }
    .command {
      display: flex;
      align-items: center;
      gap: 1rem;
      padding: 0.75rem 1rem;
      background: var(--surface);
      border-radius: var(--radius-sm);
    }
    .command code {
      min-width: 200px;
      font-weight: 600;
    }
    .command span {
      color: var(--text-muted);
      font-size: 0.9rem;
    }
    .projects-section {
      margin-bottom: 2.5rem;
    }
  `]
})
export class DashboardComponent implements OnInit {
  status: any = null;
  projects: Project[] = [];
  loading = true;

  constructor(private api: ApiService) {}

  ngOnInit(): void {
    this.api.getStatus().subscribe({
      next: (s) => this.status = s,
      error: () => this.status = { session: 'offline', active_project: 'none' }
    });

    this.api.getProjects().subscribe({
      next: (p) => { this.projects = p; this.loading = false; },
      error: () => this.loading = false,
    });
  }

  getEnvs(project: Project): string[] {
    return Object.keys(project.environments || {});
  }

  getKeyCount(project: Project): number {
    let count = 0;
    for (const env of Object.values(project.environments || {})) {
      count += Object.keys(env.secrets || {}).length;
    }
    return count;
  }
}
