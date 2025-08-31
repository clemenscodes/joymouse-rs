import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';

import { invoke } from '@tauri-apps/api/core';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent { 
  async joymouse() {
    await invoke('joymouse')
  }
}
