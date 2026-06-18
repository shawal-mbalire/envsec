export interface Project {
  name: string;
  environments: Record<string, Environment>;
}

export interface Environment {
  secrets: Record<string, Secret>;
}

export interface Secret {
  value: string;
  created: string;
  updated: string;
}

export interface Session {
  authenticated_at: string;
  expires_at: string;
  duration_secs: number;
}

export interface StatusInfo {
  vault: string;
  session: string;
  session_duration: string;
  clipboard_clear: string;
  active_project: string;
}

export interface ProjectConfig {
  project: string;
  environment: string;
}
