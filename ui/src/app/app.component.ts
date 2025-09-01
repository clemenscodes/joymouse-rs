import { Component } from '@angular/core';
import { HlmButton } from '@spartan-ng/helm/button';
import { invoke } from '@tauri-apps/api/core';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [HlmButton],
  templateUrl: './app.component.html',
})
export class AppComponent {
  async joymouse() {
    await invoke('joymouse')
  }
}
