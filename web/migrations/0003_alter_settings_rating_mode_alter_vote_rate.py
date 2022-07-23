# Generated by Django 4.0.6 on 2022-07-23 19:35

from django.db import migrations, models


class Migration(migrations.Migration):

    dependencies = [
        ('web', '0002_rename_type_settings_rating_mode_and_more'),
    ]

    operations = [
        migrations.AlterField(
            model_name='settings',
            name='rating_mode',
            field=models.TextField(choices=[('None', 'Default'), ('disabled', 'Disabled'), ('updown', 'Updown'), ('stars', 'Stars')], default=None, null=True, verbose_name='Система рейтинга'),
        ),
        migrations.AlterField(
            model_name='vote',
            name='rate',
            field=models.FloatField(verbose_name='Оценка'),
        ),
    ]
