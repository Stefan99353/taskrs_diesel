import {Component, OnInit} from '@angular/core';
import {AuthService} from '../../core/services/auth.service';
import {Router} from '@angular/router';

@Component({
    selector: 'app-page-scaffold',
    templateUrl: './page-scaffold.component.html',
    styleUrls: ['./page-scaffold.component.scss'],
})
export class PageScaffoldComponent implements OnInit {

    constructor(
        public authService: AuthService,
        private router: Router
    ) {
    }

    ngOnInit(): void {
    }

    logout(): void {
        this.authService.logout().subscribe(x => {
            this.router.navigateByUrl('/login')
        })
    }
}
