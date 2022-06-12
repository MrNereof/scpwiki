# Generated by Django 4.0.4 on 2022-06-09 09:21

from django.db import migrations, models


class Migration(migrations.Migration):

    dependencies = [
        ('system', '0003_remove_profile_id_alter_profile_user'),
    ]

    operations = [
        migrations.AlterField(
            model_name='profile',
            name='avatar',
            field=models.ImageField(blank=True, default='/static/images/default_avatar.png', upload_to='avatars', verbose_name='Аватар'),
        ),
        migrations.AlterField(
            model_name='profile',
            name='bio',
            field=models.TextField(blank=True, verbose_name='Описание'),
        ),
    ]