$(function () {
    
});

//ハンバーガーメニューの表示・非表示の処理
function hamburgerMenuShow(){
    if ($('.nav-btn').hasClass('open')) {
        $('.nav-btn').removeClass('open');
        $('#navi').removeClass('open-menu');
    } else {
        $('.nav-btn').addClass('open');
        $('#navi').addClass('open-menu');
    }
}

//スレッド追加エリアの表示・非表示の処理
function hideThreadAddArea() {
    var dialog = document.getElementById("addThreadDialog");
    dialog.style.display = "none";
    return;
}
function showThreadAddArea() {
    var dialog = document.getElementById("addThreadDialog");
    dialog.style.display = "block";
    return;
}

//スレッド追加エリアの表示・非表示の処理
function hideAccountDeleteArea() {
    var dialog = document.getElementById("deleteAccountDialog");
    dialog.style.display = "none";
    return;
}
function showAccountDeleteArea() {
    var dialog = document.getElementById("deleteAccountDialog");
    dialog.style.display = "block";
    return;
}