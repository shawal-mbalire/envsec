import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { ApiService } from '../../services/api.service';
import { Project } from '../../types';

@Component({
  selector: 'app-projects',
  standalone: true,
  imports: [CommonModule, FormsModule],
  template: `
    <div class="projects">
      <header class="page-header">
        <h1>Projects</h1>
        <p class="subtitle">Manage your projects and secrets</p>
      </header>

      @if (loading) {
        <div class="empty">Loading...</div>
      } @else if (projects.length === 0) {
        <div class="empty">
          <p>No projects found.</p>
          <code>envsec use &lt;project&gt; &lt;environment&gt;</code>
        </div>
      } @else {
        <div class="project-grid">
          @for (project of projects; track project.name) {
            <div class="project-block">
              <div class="project-header">
                <h2>{{ project.name }}</h2>
                <span class="key-count">{{ getKeyCount(project) }} keys</span>
              </div>

              @for (env of getEnvs(project); track env) {
                <div class="env-block">
                  <div class="env-header">
                    <span class="env-name">{{ env }}</span>
                    <span class="env-count">{{ getEnvKeyCount(project, env) }} secrets</span>
                  </div>

                  <table class="secrets-table">
                    <thead>
                      <tr>
                        <th>Key</th>
                        <th>Value</th>
                        <th>Updated</th>
                      </tr>
                    </thead>
                    <tbody>
                      @for (entry of getSecretEntries(project, env); track entry.key) {
                        <tr>
                          <td class="key-cell">{{ entry.key }}</td>
                          <td class="value-cell">
                            <code>{{ entry.masked }}</code>
                          </td>
                          <td class="date-cell">{{ entry.updated }}</td>
                        </tr>
                      }
                    </tbody>
                  </table>
                </div>
              }
            </div>
          }
        </div>
      }
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
    .empty {
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: var(--radius);
      padding: 2rem;
      text-align: center;
      color: var(--text-muted);
    }
    .empty code {
      display: inline-block;
      margin-top: 0.5rem;
    }
    .project-grid {
      display: grid;
      gap: 1.5rem;
    }
    .project-block {
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: var(--radius);
      overflow: hidden;
    }
    .project-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 1rem 1.25rem;
      border-bottom: 1px solid var(--border);
    }
    .project-header h2 {
      font-size: 1.1rem;
      font-weight: 700;
    }
    .key-count {
      color: var(--text-muted);
      font-size: 0.85rem;
    }
    .env-block {
      border-bottom: 1px solid var(--border);
    }
    .env-block:last-child {
      border-bottom: none;
    }
    .env-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 0.75rem 1.25rem;
      background: var(--bg);
    }
    .env-name {
      font-weight: 600;
      font-size: 0.9rem;
      color: var(--accent);
    }
    .env-count {
      color: var(--text-dim);
      font-size: 0.8rem;
    }
    .secrets-table {
      width: 100%;
    }
    .secrets-table th {
      padding: 0.5rem 1.25rem;
      font-size: 0.75rem;
    }
    .secrets-table td {
      padding: 0.6rem 1.25rem;
      font-size: 0.9rem;
    }
    .key-cell {
      font-weight: 500;
    }
    .value-cell code {
      font-size: 0.85rem;
    }
    .date-cell {
      color: var(--text-muted);
      font-size: 0.85rem;
    }
  `]
})
export class ProjectsComponent implements OnInit {
  projects: Project[] = [];
  loading = true;

  constructor(private api: ApiService) {}

  ngOnInit(): void {
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

  getEnvKeyCount(project: Project, env: string): number {
    return Object.keys(project.environments?.[env]?.secrets || {}).length;
  }

  getSecretEntries(project: Project, env: string): { key: string; masked: string; updated: string }[] {
    const secrets = project.environments?.[env]?.secrets || {};
    return Object.entries(secrets).map(([key, secret]) => ({
      key,
      masked: this.mask(secret.value),
      updated: new Date(secret.updated).toLocaleDateString(),
    }));
  }

  private mask(value: string): string {
    if (value.length <= 3) return '***';
    return value.substring(0, 3) + '***';
  }
}
