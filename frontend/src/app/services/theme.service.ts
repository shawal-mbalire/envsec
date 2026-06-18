import { Injectable } from '@angular/core';
import { BehaviorSubject } from 'rxjs';

export type Theme = 'light' | 'dark';

@Injectable({
  providedIn: 'root'
})
export class ThemeService {
  private theme$ = new BehaviorSubject<Theme>(this.getInitialTheme());

  get current$() {
    return this.theme$.asObservable();
  }

  get current(): Theme {
    return this.theme$.value;
  }

  toggle(): void {
    const next = this.theme$.value === 'light' ? 'dark' : 'light';
    this.set(next);
  }

  set(theme: Theme): void {
    this.theme$.next(theme);
    document.documentElement.className = theme;
    localStorage.setItem('envsec-theme', theme);
  }

  private getInitialTheme(): Theme {
    const stored = localStorage.getItem('envsec-theme') as Theme;
    if (stored) return stored;
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
  }
}
