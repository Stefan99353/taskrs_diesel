import {Component, OnInit} from '@angular/core';
import {FormBuilder, FormGroup, Validators} from '@angular/forms';
import {AuthService} from '../../core/services/auth.service';
import {HttpErrorResponse} from '@angular/common/http';
import {Router} from '@angular/router';

@Component({
    selector: 'app-login',
    templateUrl: './login.component.html',
    styleUrls: ['./login.component.scss'],
})
export class LoginComponent implements OnInit {
    public loginError: string | null = null;

    public loginForm: FormGroup = this.fb.group({
        email: ['', Validators.required],
        password: ['', Validators.required],
    });

    constructor(
        private fb: FormBuilder,
        private router: Router,
        private authService: AuthService,
    ) {
    }

    ngOnInit(): void {
    }

    login(): void {
        const values = this.loginForm.value;

        if (this.loginForm.valid) {
            this.authService
                .login(values.email, values.password)
                .subscribe(
                    tokens => {
                        this.router.navigateByUrl('/home');
                    },
                    (error: HttpErrorResponse) => {
                        if (error.status === 400) {
                            // email or password wrong or user deactivated
                            this.loginError = 'Either email and password are wrong or the User is deactivated.';
                        }
                    },
                );
        }
    }
}
