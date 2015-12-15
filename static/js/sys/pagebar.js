var Com = function(config) {
	var self = this;
	self.config = config;
    self.parent = self.config.parent;
	//子页面
	self.cr = {
	};
	self.init();
};

Com.prototype.init = function() {
	var self = this;
    self.cur = self.skip/self.limit + 1; //current page
    var index_array = self.get_index_array();
    console.log(index_array);
};

Com.prototype.get_html = function(index_array) {
    var self = this;
    var html = '<nav><ul class="pagination">'

    html += '</ur></nav>';
    return html;
}

Com.prototype.get_index_array = function() {
    var self = this;
    var add = self.config.add;
    var page_count = parseInt(add.total/add.limit) + 1;
    var array = [];
    if(page_count < 7) {
        for(var i = 0; i < page_count; i++) {
            array.push(i + 1);
        }
    } else {
        if(self.cur < 4 || page_count - self.cur < 3) {
           array.push(1);array.push(2);array.push(3);
           array.push(-1);
           array.push(page_count - 2);array.push(page_count - 1);array.push(page_count);
        } else {
           array.push(1);array.push(-1);
           array.push(self.cur - 1);
           array.push(self.cur);
           array.push(self.cur + 1);
           array.push(page_count - 1);array.push(page_count);
        }
    }
    return array;
}

return Com;
