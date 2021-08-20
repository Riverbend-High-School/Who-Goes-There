![Riverbend High School](https://i.imgur.com/kvyId31t.png)

# **RHS Library Who Goes There?** Django REST API

![Python     .8](https://img.shields.io/badge/Python-3.8-%232D44A4?style=flat)
![GPLv3 License](https://img.shields.io/badge/License-GPLv3-%232D44A4?style=flat)
![Version 0.1.0](https://img.shields.io/badge/Version-v0.1.0-%232D44A4?style=flat)

This repository contains the backend server code for the student monitoring and session tracking system used in the Riverbend Library. This api is used to implement a simple signin/out system to make sure students are accounted for at all times, including in an emergency.

This backend is running on Python 3.8 using the [Django REST Framework](https://www.django-rest-framework.org/). It is designed to be used with a separate frontend component. The RHS Library uses a react based implementation, see [Github Repo Here](https://github.com/Riverbend-High-School/wgt-frontend/).

Created by [Gabriel Hogan](https://gabrielhogan.com).


## Installation
All commands assume that you are at the root of the `wgt-backend` repository that you cloned.
1. Ensure you have both Python and pip up to date **(Minimum Python Version 3.8)**. (Using a `venv` is recommended)
    ```
    python -m pip install --upgrade pip
    ```

2. Install all project dependencies.
    ```
    pip install -r requirements.txt
    ```

3. Create and populate the `.env` file.
    ```
    cp .env.example .env
    ```

    <details>
    <summary>Environment Field Descriptions</summary>

    *All strings must be surrounded with double quotes. Integers and booleans must be on their own.*
    | Field                               | Description                                                          | Example              |
    | ----------------------------------- | -------------------------------------------------------------------- | -------------------- |
    | `DEV_ENV`                           |  Enables debug mode. **Must be `False` in production**               | `True`               |
    | `ALLOWED_HOSTS`                     |  Comma separated list of domains and IPs that the server will run on | `"wgt-api.rhslib.com"` |
    | `SECRET_KEY`                        |  Django secret key. Can be generated [here](https://djecrety.ir/)    |                      |
    | `SENTRY_DSN` **[Optional]**         |  Sentry DSN for error logging                                        |                      |
    | `STATIC_ROOT` **[Optional]**        |  Root directory for static files. Defaults to `./static`             | `"/home/.../static"` |
    | `MEDIA_ROOT` **[Optional]**         |  Root directory for uploaded files. Defaults to `./media`            | `"/home/.../media"`  |
    </details>

4. Create database tables.
    ```
    python manage.py makemigrations core students visits
    python manage.py migrate
    ```

5. Create access to the Django admin panel. (Accessible at `/admin`)
    ```
    python manage.py createsuperuser
    ```

## Authentication
Authentication with the REST API is done through **JSON Web Tokens**. Please note that JWTs are not encrypted and may contain sensitive information.

## License
Distributed under the GPLv3 License. See [LICENSE](LICENSE) for more information.

## Contact
Gabriel Hogan - me@gabrielhogan.com
