import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable, of } from 'rxjs';
import { Project, ProjectConfig, StatusInfo } from './types';

@Injectable({
  providedIn: 'root'
})
export class ApiService {
  private baseUrl = '/api';

  constructor(private http: HttpClient) {}

  getStatus(): Observable<StatusInfo> {
    return this.http.get<StatusInfo>(`${this.baseUrl}/status`);
  }

  getProjects(): Observable<Project[]> {
    return this.http.get<Project[]>(`${this.baseUrl}/projects`);
  }

  getProjectConfig(): Observable<ProjectConfig> {
    return this.http.get<ProjectConfig>(`${this.baseUrl}/config`);
  }

  setProject(project: string, environment: string): Observable<void> {
    return this.http.post<void>(`${this.baseUrl}/use`, { project, environment });
  }

  getSecrets(project: string, environment: string): Observable<Record<string, { masked: string; updated: string }>> {
    return this.http.get<Record<string, { masked: string; updated: string }>>(
      `${this.baseUrl}/secrets/${project}/${environment}`
    );
  }

  setSecret(project: string, environment: string, key: string, value: string): Observable<void> {
    return this.http.post<void>(`${this.baseUrl}/secrets`, { project, environment, key, value });
  }

  removeSecret(project: string, environment: string, key: string): Observable<void> {
    return this.http.delete<void>(`${this.baseUrl}/secrets/${project}/${environment}/${key}`);
  }

  importEnv(project: string, environment: string, content: string): Observable<{ count: number }> {
    return this.http.post<{ count: number }>(`${this.baseUrl}/import`, { project, environment, content });
  }

  exportEnv(project: string, environment: string): Observable<string> {
    return this.http.get(`${this.baseUrl}/export/${project}/${environment}`, { responseType: 'text' });
  }
}
