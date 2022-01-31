from datetime import timedelta
from django.db.models import Sum, Q, DurationField
from django.db.models.functions import Coalesce, Cast
from django.utils import timezone

from ..visits.models import Visit
from ..students.models import Student


def annotate_hours(query):
    """
    Annotates given QuerySet with controlling hours for the
    current (curr_hours), previous (prev_hours), and
    penultimate (prev_prev_hours) months.
    """
    MONTH_NOW = timezone.now().month
    YEAR_NOW = timezone.now().year
    CURR_MONTH = (Q(student_visits__start__month=MONTH_NOW)
                  & Q(student_visits__start__year=YEAR_NOW))
    PREV_MONTH = (Q(student_visits__start__month=MONTH_NOW - 1 if MONTH_NOW > 1 else 12)
                  & Q(student_visits__start__year=YEAR_NOW if MONTH_NOW > 1 else YEAR_NOW - 1))
    PREV_PREV_MONTH = (Q(student_visits__start__month=MONTH_NOW - 2 if MONTH_NOW > 2 else 12 if MONTH_NOW > 1 else 11)
                       & Q(student_visits__start__year=YEAR_NOW if MONTH_NOW > 2 else YEAR_NOW - 1))

    return query.annotate(
        curr_hours=Coalesce(Sum('student_visits__duration', filter=CURR_MONTH), Cast(timedelta(), DurationField())),
        prev_hours=Coalesce(Sum('student_visits__duration', filter=PREV_MONTH), Cast(timedelta(), DurationField())),
        prev_prev_hours=Coalesce(Sum('student_visits__duration', filter=PREV_PREV_MONTH), Cast(timedelta(), DurationField())),
    )


def get_student_hours():
    """
    Returns query set of active users annotated with controlling
    hours for the current (curr_hours), previous (prev_hours),
    and penultimate (prev_prev_hours) months.
    """
    return annotate_hours(Student.objects.all())


def get_top_students():
    """
    Returns query set of active users annotated with controlling
    hour sums for the current month (hours) sorted by most
    controlling hours (controllers with no hours are not included).
    """
    SAME_MONTH = Q(student_visits__start__month=timezone.now().month)
    SAME_YEAR = Q(student_visits__start__year=timezone.now().year)

    students = Student.objects.all()
    students = students.annotate(hours=Sum('student_visits__duration', filter=SAME_MONTH & SAME_YEAR))

    return students.exclude(hours__isnull=True).order_by('-hours')


def get_daily_statistics(year, student=None):
    """
    Returns a query dictionary of every day of the
    given year annotated with the controlling hours
    for that day.
    """
    visits = Visit.objects.filter(start__year=year)
    if student:
        visits = visits.filter(student=student)
    return visits.extra({'day': 'date(start)'}).values('day').annotate(value=Sum('duration'))
