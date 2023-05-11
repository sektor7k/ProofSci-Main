function previewAvatar(event) {
    var reader = new FileReader();
    reader.onload = function () {
        var avatarPreview = document.getElementById('avatar-preview');
        avatarPreview.src = reader.result;
    }
    reader.readAsDataURL(event.target.files[0]);
}