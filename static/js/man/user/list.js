var Com = function(config) {
	var self = this;
	self.config = config;
    self.parent = self.config.parent;
	//子页面
	self.cr = {
        'pagebar': {
            pins: self,
            parent: self.parent.children('#pagebar')
        }
	};
    self.skip = 0;
    self.limit = 4;
    self.sort = {};
    self.cond = {};
	self.init();
};

Com.prototype.init = function() {
	var self = this;
    self.dom_user_list = self.parent.find('#user_list');
    self.to_page(1);
};

Com.prototype.to_page = function(index) {
    var self = this;
    self.skip = (index - 1)*self.limit;
    var cond = JSON.stringify(self.cond);
    var sort = JSON.stringify(self.sort);
    var body = {
        cond: cond,
        sort: sort,
        offset: self.skip,
        limit: self.limit
    };
    CurSite.postDigest({cmd:"U03"}, body, function(err, back_body)
    {
       self.dom_user_list.html(self.get_table(back_body.data));
       self.cr.pagebar.add = {
           skip: self.skip,
           limit: self.limit,
           total: back_body.count
       }
       CurSite.to_page(self.cr.pagebar, "sys_pagebar");
    });
}

Com.prototype.get_table = function(data) {
    var self = this;
    var html = '<table class="table table-striped table-hover">';
    html += '<thead><tr><td>用户名</td><td>类型</td><td>注册时间</td></tr></thead>';
    html += '<tbody>';
    for(var i = 0; i < data.length; i++) {
        html += '<tr><td>' + data[i].username + '</td><td>' + data[i].type + '</td><td>' + CurSite.getDateStr(data[i].reg_time*1000) + '</td></tr>';
    }
    html += '</tbody>';
    html += '</table>';
    return html;
}

return Com;
