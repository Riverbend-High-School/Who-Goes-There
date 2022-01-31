import os
from datetime import timedelta
from pathlib import Path
from dotenv import load_dotenv
import sentry_sdk
from sentry_sdk.integrations.django import DjangoIntegration

# Build paths inside the project like this: BASE_DIR / 'subdir'.
BASE_DIR = Path(__file__).resolve().parent.parent

# Loads environment variables from .env file
load_dotenv(BASE_DIR / '.env')

# Initalize Sentry.io SDK for error handling
if os.getenv('DEV_ENV', '') == 'False' and os.getenv('SENTRY_DSN'):
    sentry_sdk.init(
        dsn=os.getenv('SENTRY_DSN'),
        integrations=[DjangoIntegration()],
        traces_sample_rate=0.75,
        send_default_pii=True,
    )


# Get a bunch of information from the env file
SECRET_KEY = os.getenv('SECRET_KEY', 'evrf9qiqh6dnaad+i10r!vcaz2dx1@37779vp0sh5x1147f%pg')

DEBUG = os.getenv('DEV_ENV', '') == 'True'

ALLOWED_HOSTS = os.getenv('ALLOWED_HOSTS', '').split(',')

SECURE_HSTS_SECONDS = os.getenv('DEV_ENV', '') == 'False'

SECURE_SSL_REDIRECT = os.getenv('DEV_ENV', '') == 'False'

SECURE_HSTS_INCLUDE_SUBDOMAINS = os.getenv('DEV_ENV', '') == 'False'

SECURE_HSTS_PRELOAD = os.getenv('DEV_ENV', '') == 'False'


# Application definition

INSTALLED_APPS = [
    # Django Apps
    'django.contrib.admin',
    'django.contrib.auth',
    'django.contrib.contenttypes',
    'django.contrib.sessions',
    'django.contrib.messages',
    'django.contrib.staticfiles',

    # Third-Party Apps
    'corsheaders',
    'rest_framework',
    'rest_framework.authtoken',

    # Project Apps
    #'apps.dashboard',
    'apps.core',
    'apps.students',
    'apps.visits',
    'apps.portals',
    'apps.statistics',
]

REST_FRAMEWORK = {
    'DEFAULT_AUTHENTICATION_CLASSES': [
        'rest_framework.authentication.TokenAuthentication',
        'wgt_backend.authentication.TokenAuthSupportQueryString'
    ],
}

MIDDLEWARE = [
    'django.middleware.security.SecurityMiddleware',
    'django.contrib.sessions.middleware.SessionMiddleware',
    'corsheaders.middleware.CorsMiddleware',
    'django.middleware.common.CommonMiddleware',
    'django.contrib.auth.middleware.AuthenticationMiddleware',
    'django.contrib.messages.middleware.MessageMiddleware',
    'django.middleware.clickjacking.XFrameOptionsMiddleware',
]

CORS_ALLOWED_ORIGINS = [
    'https://wgt.gabrielhogan.com',
    'http://wgt.gabrielhogan.devel',
    'https://api-wgt.gabrielhogan.com',
    'http://localhost:3000',
    'http://127.0.0.1:3000',
]

CORS_ALLOW_HEADERS = [
    'accept',
    'accept-encoding',
    'authorization',
    'content-type',
    'dnt',
    'origin',
    'user-agent',
    'cache-control',
    'x-csrftoken',
    'x-requested-with',
]

ROOT_URLCONF = 'wgt_backend.urls'

TEMPLATES = [
    {
        'BACKEND': 'django.template.backends.django.DjangoTemplates',
        'DIRS': [],
        'APP_DIRS': True,
        'OPTIONS': {
            'context_processors': [
                'django.template.context_processors.debug',
                'django.template.context_processors.request',
                'django.contrib.auth.context_processors.auth',
                'django.contrib.messages.context_processors.messages',
            ],
        },
    },
]

WSGI_APPLICATION = 'wgt_backend.wsgi.application'


# Database
# https://docs.djangoproject.com/en/3.2/ref/settings/#databases

DATABASES = {
    'default': {
        'ENGINE': 'django.db.backends.sqlite3',
        'NAME': BASE_DIR / 'db.sqlite3',
    }
}


# Password validation
# https://docs.djangoproject.com/en/3.2/ref/settings/#auth-password-validators

AUTH_PASSWORD_VALIDATORS = [
    {
        'NAME': 'django.contrib.auth.password_validation.UserAttributeSimilarityValidator',
    },
    {
        'NAME': 'django.contrib.auth.password_validation.MinimumLengthValidator',
    },
    {
        'NAME': 'django.contrib.auth.password_validation.CommonPasswordValidator',
    },
    {
        'NAME': 'django.contrib.auth.password_validation.NumericPasswordValidator',
    },
]


# Internationalization
# https://docs.djangoproject.com/en/3.2/topics/i18n/

LANGUAGE_CODE = 'en-us'

TIME_ZONE = 'UTC'

USE_I18N = True

USE_L10N = True

USE_TZ = True


# Static files (CSS, JavaScript, Images)
# https://docs.djangoproject.com/en/3.2/howto/static-files/

STATIC_URL = '/static/'

STATIC_ROOT = os.getenv('STATIC_ROOT', BASE_DIR / 'static')

MEDIA_URL = '/media/'

MEDIA_ROOT = os.getenv('MEDIA_ROOT', BASE_DIR / 'media')

# Email configuration
EMAIL_HOST = os.getenv('EMAIL_HOST')

EMAIL_PORT = os.getenv('EMAIL_PORT')

EMAIL_HOST_USER = os.getenv('EMAIL_HOST_USER')

EMAIL_HOST_PASSWORD = os.getenv('EMAIL_HOST_PASSWORD')

EMAIL_USE_TLS = os.getenv('EMAIL_USE_TLS')

# Miscellaneous
BLEACH_ALLOWED_TAGS = [
    'h1', 'h2', 'h3', 'h4', 'h5', 'h6',
    'pre', 'span', 'img', 'strong', 'blockquote',
    'p', 'a', 'u', 's', 'em', 'br', 'ul', 'ol', 'li',
]