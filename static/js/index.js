var Com = function(config) {
    var self = this;
    self.config = config;
    self.parent = config.parent;
    //子页面
    self.cr = {
        'main': {
            pins: self,
            parent: $("#content")
        }
    };
    self.dom_head = self.parent.find('#book_types');
    self.init();
};

Com.prototype.init = function() {
    var self = this;
    CurSite.postUnDigest({cmd:"GBT01"}, {}, function(err, data){
        if(data) {
            var html = '';
            var rows = data.data;
            for(var i = 0; i < rows.length; i++) {
                var set = rows[i];
                html += self.get_html(set);
            }
            self.dom_head.append(html);
        }
    });
};

Com.prototype.get_html = function(row) {
    var self = this;
    var html = 
        '<div class="navbar-header">' + 
          '<a class="navbar-brand" href="./index.html">' + row.name + '</a>' + 
        '</div>';
    return html;
}

return Com;
