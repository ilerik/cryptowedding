$(document).ready(function() {
    $('.submit-form').click(function() {
      // Check is the form filled correct
      if ($( this ).hasClass('invalid')) return;

      // Form data
      var data = {};
      var idx = 0;
      $(".req-input input, .req-input textarea").each(function(){
          data[idx] = $(this).val();
          idx++;
      });
      var data_str = JSON.stringify(data);

      // Send data
      var xhr = new XMLHttpRequest();
      var url = "http://144.76.106.46/api";
      xhr.open('POST', url, true);
      console.log(xhr.readyState);
      xhr.send(data_str);
      console.log(xhr.readyState);
      xhr.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
          var response = JSON.parse(this.responseText);
          alert("Your multisig wallet: " + response.priest_address + " BITCH!");
        }
      };

      $('#multisig-form').each(function(){
          this.reset();
      });
    });
});
