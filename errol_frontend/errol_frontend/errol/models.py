from django.contrib.postgres import fields
from django.db import models

# Create your models here.
# This is an auto-generated Django model module.
# You'll have to do the following manually to clean this up:
#   * Rearrange models' order
#   * Make sure each model has one field with primary_key=True
#   * Make sure each ForeignKey has `on_delete` set to the desired behavior.
#   * Remove `managed = False` lines if you wish to allow Django to create, modify, and delete the table
# Feel free to rename the models, but don't rename db_table values or field names.
from django.urls import reverse


class DieselSchemaMigrations(models.Model):
    version = models.CharField(primary_key=True, max_length=50)
    run_on = models.DateTimeField()

    class Meta:
        managed = False
        db_table = '__diesel_schema_migrations'


class GithubIssues(models.Model):
    github_repo = models.ForeignKey('GithubRepos', models.DO_NOTHING, blank=True, null=True)
    number = models.IntegerField()
    title = models.CharField(max_length=255)
    body = models.TextField()
    labels = models.TextField()
    url = models.TextField()
    data = models.TextField()
    created_at = models.DateTimeField(blank=True, null=True)
    updated_at = models.DateTimeField(blank=True, null=True)
    deleted_at = models.DateTimeField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'github_issues'


class GithubIssuesFetches(models.Model):
    github_repo = models.ForeignKey('GithubRepos', models.DO_NOTHING, blank=True, null=True)
    repo = models.CharField(max_length=255)
    data = models.TextField()
    created_at = models.DateTimeField(blank=True, null=True)
    updated_at = models.DateTimeField(blank=True, null=True)
    deleted_at = models.DateTimeField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'github_issues_fetches'


class GithubPulls(models.Model):
    github_repo = models.ForeignKey('GithubRepos', models.DO_NOTHING, blank=True, null=True)
    number = models.IntegerField()
    title = models.CharField(max_length=255)
    repo = models.CharField(max_length=255)
    body = models.TextField()
    labels = models.TextField()
    url = models.TextField()
    data = models.TextField()
    created_at = models.DateTimeField(blank=True, null=True)
    updated_at = models.DateTimeField(blank=True, null=True)
    deleted_at = models.DateTimeField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'github_pulls'


class GithubPullsFetches(models.Model):
    github_repo = models.ForeignKey('GithubRepos', models.DO_NOTHING, blank=True, null=True)
    repo = models.CharField(max_length=255)
    data = models.TextField()
    created_at = models.DateTimeField(blank=True, null=True)
    updated_at = models.DateTimeField(blank=True, null=True)
    deleted_at = models.DateTimeField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'github_pulls_fetches'


class GithubRepos(models.Model):
    repo = models.CharField(max_length=255)
    url = models.TextField()
    created_at = models.DateTimeField(blank=True, null=True)
    updated_at = models.DateTimeField(blank=True, null=True)
    deleted_at = models.DateTimeField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'github_repos'


class Rules(models.Model):
    name = models.CharField(max_length=255)
    reply_to = models.TextField(blank=True, null=True)
    authors = fields.ArrayField(models.CharField(max_length=255), blank=True, null=True)  # This field type is a guess.
    branches = fields.ArrayField(models.CharField(max_length=255), blank=True, null=True)  # This field type is a guess.
    paths = fields.ArrayField(models.CharField(max_length=255), blank=True, null=True)  # This field type is a guess.
    projects = fields.ArrayField(models.CharField(max_length=255), blank=True, null=True)  # This field type is a guess.
    to = fields.ArrayField(models.CharField(max_length=255), blank=True, null=True)  # This field type is a guess.

    class Meta:
        managed = False
        db_table = 'rules'

    def get_absolute_url(self):
        return reverse("rules:detail", kwargs={"pk": self.pk})
