from django.urls import path

from errol_frontend.errol import views

app_name = "rules"
urlpatterns = [
    path("", view=views.rule_list_view, name="list"),
    path("~redirect/", view=views.rule_redirect_view, name="redirect"),
    path("~create/", view=views.rule_create_view, name="create"),
    path("<str:pk>/~update/", view=views.rule_update_view, name="update"),
    path("<str:pk>/", view=views.rule_detail_view, name="detail"),
]
