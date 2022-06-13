from django.contrib.auth.mixins import LoginRequiredMixin
from django.views.generic import DetailView, UpdateView
from django.shortcuts import resolve_url

from system.models import User


class ProfileView(DetailView):
    model = User
    slug_field = "username"


class MyProfileView(LoginRequiredMixin, ProfileView):
    def get_object(self, queryset=None):
        return self.request.user


class ChangeProfileView(LoginRequiredMixin, UpdateView):
    model = User
    fields = ['username', 'email', 'first_name', 'last_name', "bio", "avatar"]

    def get_success_url(self):
        return resolve_url("profile")

    def get_object(self, queryset=None):
        return self.request.user
