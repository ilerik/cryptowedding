// add script for auto-collapsing after section is choosen
$(function() {
    $('.nav a').on('click', function(){
        if($('.navbar-toggle').css('display') !='none'){
            $('.navbar-toggle').trigger( 'click' );
            $(this).next().focus();
        }
    });
});

$(function() {
    $( '.menuItem').click(function() {
        $( '.menuItem' ).not($( this )).removeClass( 'active' );
        $( this ).toggleClass( 'active' );
    });
});
