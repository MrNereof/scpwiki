# Generated by Django 4.0.4 on 2022-07-16 11:58

from django.db import migrations, models


class Migration(migrations.Migration):

    dependencies = [
        ('web', '0006_alter_file_deleted_at_alter_file_deleted_by'),
    ]

    operations = [
        migrations.AddField(
            model_name='site',
            name='icon',
            field=models.ImageField(blank=True, null=True, upload_to='-/sites', verbose_name='Иконка'),
        ),
    ]
