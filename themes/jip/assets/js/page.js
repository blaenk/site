function userWantsDarkMode() {
  return localStorage.theme === 'dark' || (!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches);
}

function setTheme() {
  const buttons = document.querySelectorAll(".theme_btn");

  if (userWantsDarkMode()) {
    document.documentElement.classList.add('dark');
    buttons.forEach(function(el) {
      el.src = "/images/light-mode.svg";
    });
  } else {
    document.documentElement.classList.remove('dark');
    buttons.forEach(function(el) {
      el.src = "/images/dark-mode.svg";
    });
  }
}

setTheme();

window.addEventListener('storage', function() {
  setTheme();
});

$(function() {
  setTheme();

  $(".theme_btn").click(function(event) {
    var wasDark = userWantsDarkMode();

    if (wasDark) {
      localStorage.theme = "light";
    } else {
      localStorage.theme = "dark";
    }

    setTheme();
  });

  var mobilenav = $('#mobile-nav');

  $('html').click(function(){
    mobilenav.find('.on').each(function(){
      $(this).removeClass('on').next().hide();
    });
  });

  mobilenav.on('click', '.menu .button', function(e){
    if (!$(this).hasClass('on')) {
      var width = $(this).width() + 70;
      $(this).addClass('on').next().css({width: width}).show();
    } else {
      $(this).removeClass('on').next().hide();
    }
  }).on('click', '.search .button', function(e){
    if (!$(this).hasClass('on')){
      var width = mobilenav.width() - 51;
      mobilenav.children('.menu').children().eq(0).removeClass('on').next().hide();
      $(this).addClass('on').next().show().css({width: width}).children().children().eq(0).focus();
    } else {
      $(this).removeClass('on').next().hide().children().children().eq(0).val('');
    }
  }).click(function(e){
    e.stopPropagation();
  });

  var $nav = $('#main-nav .main');

  var toggleForm = function(button, form) {
    button.click(function() {
      if (form.is(':visible')) {
        form.fadeOut('fast');
      } else {
        form.fadeIn('fast');
        form.find('input').focus();
      }
    });

    form.find('input').keyup(function(e) {
      if (e.keyCode == 27) {
        form.fadeOut('fast');
      }
    });
  };

  toggleForm($('#search_btn'), $('.desk_search'));

  var $footnotes = $('.footnotes > ol > li');

  $('a[href^="#fn:"]')
    .attr('title', 'read footnote')
    .click(function() {
      $footnotes.stop(true, true);

      var note = $(this).attr('href');
      $footnotes.not(note.replace(/:/, "\\:"))
        .css({ opacity: 0.1 })
        .animate({ opacity: 1.0 }, 15000, 'linear');
    });

  $('a[href^="#fnref"]')
    .html('<i class="fas fa-level-up-alt"></i>')
    .click(function() {
      $footnotes.stop(true, true);
    });

  var res = $('#toc').toc({ content: 'div.entry-content', headings: 'h1,h2,h3,h4,h5' });

  $('article.post h1[id], article.post h2[id], article.post h3[id], article.post h4[id], article.post h5[id]').each(function() {
    var text = $(this).text();
    $(this).empty();
    $(this).append('<span class="hash">#</span>');
    $(this).append('<a href="#' + this.id + '" class="header-link">' + text + '</a>');
  });
});
