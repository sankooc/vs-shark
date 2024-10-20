class Parser {
    input: string;
    pos: number;
    constructor(input) {
        this.input = input;
        this.pos = 0;
    }

    // 解析表达式
    parse() {
        return this.parseExpression();
    }

    // 解析表达式 (包含 && 和 ||)
    parseExpression() {
        let expr = this.parseTerm();

        while (this.peek() === '&&' || this.peek() === '||') {
            let operator = this.consume(undefined);
            let right = this.parseTerm();
            expr = { type: 'Binary', left: expr, operator: operator, right: right };
        }

        return expr;
    }

    // 解析单个term (处理括号或比较操作)
    parseTerm(): any {
        if (this.peek() === '(') {
            this.consume(undefined);  // 消耗 '('
            let expr = this.parseExpression();
            if (this.peek() !== ')') {
                throw new Error('Missing closing parenthesis');
            }
            this.consume(undefined);  // 消耗 ')'
            return { type: 'Group', expression: expr };
        } else {
            return this.parseComparison();
        }
    }

    // 解析比较操作 (例如 =, !=, <, >, >=, <=)
    parseComparison(): any {
        let left = this.parseValue();

        let operator = this.peekComparisonOperator();
        if (operator) {
            this.consume(operator);
            let right = this.parseValue();
            return { type: 'Binary', left: left, operator: operator, right: right };
        }
        // else {
        //     throw new Error("no value");
        // }

        return left;
    }

    // 解析值（例如变量名或数字）
    parseValue() {
        this.skipWhitespace();
        let value = '';
        while (this.pos < this.input.length && !this.isWhitespace(this.input[this.pos]) &&
               !this.isSpecialChar(this.input[this.pos])) {
            value += this.input[this.pos++];
        }
        this.skipWhitespace();
        if (value.length === 0) {
            throw new Error('Expected a value');
        }

        return { type: 'Value', value: value };
    }

    // 查看接下来的比较操作符
    peekComparisonOperator() {
        const ops = ['==', '!=', '>=', '<=', '>', '<'];
        for (let op of ops) {
            if (this.input.startsWith(op, this.pos)) {
                return op;
            }
        }
        return null;
    }

    // 查看下一个token (包括 &&, ||)
    peek() {
        this.skipWhitespace();
        if (this.input.startsWith('&&', this.pos)) return '&&';
        if (this.input.startsWith('||', this.pos)) return '||';
        if (this.input[this.pos] === '(') return '(';
        if (this.input[this.pos] === ')') return ')';
        return null;
    }

    // 消耗一个token
    consume(expected) {
        this.skipWhitespace();
        if (expected && !this.input.startsWith(expected, this.pos)) {
            throw new Error(`Expected "${expected}"`);
        }

        let token = expected || this.input[this.pos] + this.input[this.pos + 1];
        this.pos += token.length;
        return token;
    }

    // 跳过空白字符
    skipWhitespace() {
        while (this.pos < this.input.length && this.isWhitespace(this.input[this.pos])) {
            this.pos++;
        }
    }

    // 判断是否为空白字符
    isWhitespace(char) {
        return /\s/.test(char);
    }

    // 判断是否为特殊字符
    isSpecialChar(char) {
        return ['=', '!', '>', '<', '&', '|', '(', ')'].includes(char);
    }
}

// const input = "tcp.ip  && (act.count >= 1 || ppc.c < 12)";
// const parser = new Parser(input);
// const result = parser.parse();

// console.log(JSON.stringify(result, null, 2));
