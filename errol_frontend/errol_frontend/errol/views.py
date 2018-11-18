from django.contrib.auth.mixins import LoginRequiredMixin
from django.urls import reverse
from django.views.generic import CreateView, DetailView, ListView, RedirectView, UpdateView

from errol_frontend.errol.models import Rules


class RulesDetailView(LoginRequiredMixin, DetailView):
    model = Rules


rule_detail_view = RulesDetailView.as_view()


class RulesListView(LoginRequiredMixin, ListView):
    model = Rules


rule_list_view = RulesListView.as_view()


class RulesCreateView(LoginRequiredMixin, CreateView):

    model = Rules
    fields = ["name", "to", "authors", "branches", "paths", "projects"]


rule_create_view = RulesCreateView.as_view()


class RulesUpdateView(LoginRequiredMixin, UpdateView):
    model = Rules
    fields = ["name", "to", "authors", "branches", "paths", "projects"]


rule_update_view = RulesUpdateView.as_view()


class RulesRedirectView(LoginRequiredMixin, RedirectView):

    permanent = False

    def get_redirect_url(self):
        return reverse("rules:detail", kwargs={"pk": self.object.id})


rule_redirect_view = RulesRedirectView.as_view()
