import { Component } from '@angular/core';


import { invoke } from '@tauri-apps/api/core';

@Component({
    selector: 'app-root',
    imports: [],
    templateUrl: './app.component.html',
    styleUrl: './app.component.css'
})
export class AppComponent { 
  async joymouse() {
    await invoke('joymouse')
  }
}
