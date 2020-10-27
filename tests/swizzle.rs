// Generated by swizzles. Do not edit.
use glam::{vec2, vec3, vec3a, vec4, Vec2Swizzles, Vec3ASwizzles, Vec3Swizzles, Vec4Swizzles};

#[test]
fn test_vec4_swizzles() {
    let v = vec4(1.0, 2.0, 3.0, 4.0);
    assert_eq!(v.xxxx(), vec4(1.0, 1.0, 1.0, 1.0));
    assert_eq!(v.xxxy(), vec4(1.0, 1.0, 1.0, 2.0));
    assert_eq!(v.xxxz(), vec4(1.0, 1.0, 1.0, 3.0));
    assert_eq!(v.xxxw(), vec4(1.0, 1.0, 1.0, 4.0));
    assert_eq!(v.xxyx(), vec4(1.0, 1.0, 2.0, 1.0));
    assert_eq!(v.xxyy(), vec4(1.0, 1.0, 2.0, 2.0));
    assert_eq!(v.xxyz(), vec4(1.0, 1.0, 2.0, 3.0));
    assert_eq!(v.xxyw(), vec4(1.0, 1.0, 2.0, 4.0));
    assert_eq!(v.xxzx(), vec4(1.0, 1.0, 3.0, 1.0));
    assert_eq!(v.xxzy(), vec4(1.0, 1.0, 3.0, 2.0));
    assert_eq!(v.xxzz(), vec4(1.0, 1.0, 3.0, 3.0));
    assert_eq!(v.xxzw(), vec4(1.0, 1.0, 3.0, 4.0));
    assert_eq!(v.xxwx(), vec4(1.0, 1.0, 4.0, 1.0));
    assert_eq!(v.xxwy(), vec4(1.0, 1.0, 4.0, 2.0));
    assert_eq!(v.xxwz(), vec4(1.0, 1.0, 4.0, 3.0));
    assert_eq!(v.xxww(), vec4(1.0, 1.0, 4.0, 4.0));
    assert_eq!(v.xyxx(), vec4(1.0, 2.0, 1.0, 1.0));
    assert_eq!(v.xyxy(), vec4(1.0, 2.0, 1.0, 2.0));
    assert_eq!(v.xyxz(), vec4(1.0, 2.0, 1.0, 3.0));
    assert_eq!(v.xyxw(), vec4(1.0, 2.0, 1.0, 4.0));
    assert_eq!(v.xyyx(), vec4(1.0, 2.0, 2.0, 1.0));
    assert_eq!(v.xyyy(), vec4(1.0, 2.0, 2.0, 2.0));
    assert_eq!(v.xyyz(), vec4(1.0, 2.0, 2.0, 3.0));
    assert_eq!(v.xyyw(), vec4(1.0, 2.0, 2.0, 4.0));
    assert_eq!(v.xyzx(), vec4(1.0, 2.0, 3.0, 1.0));
    assert_eq!(v.xyzy(), vec4(1.0, 2.0, 3.0, 2.0));
    assert_eq!(v.xyzz(), vec4(1.0, 2.0, 3.0, 3.0));
    assert_eq!(v.xywx(), vec4(1.0, 2.0, 4.0, 1.0));
    assert_eq!(v.xywy(), vec4(1.0, 2.0, 4.0, 2.0));
    assert_eq!(v.xywz(), vec4(1.0, 2.0, 4.0, 3.0));
    assert_eq!(v.xyww(), vec4(1.0, 2.0, 4.0, 4.0));
    assert_eq!(v.xzxx(), vec4(1.0, 3.0, 1.0, 1.0));
    assert_eq!(v.xzxy(), vec4(1.0, 3.0, 1.0, 2.0));
    assert_eq!(v.xzxz(), vec4(1.0, 3.0, 1.0, 3.0));
    assert_eq!(v.xzxw(), vec4(1.0, 3.0, 1.0, 4.0));
    assert_eq!(v.xzyx(), vec4(1.0, 3.0, 2.0, 1.0));
    assert_eq!(v.xzyy(), vec4(1.0, 3.0, 2.0, 2.0));
    assert_eq!(v.xzyz(), vec4(1.0, 3.0, 2.0, 3.0));
    assert_eq!(v.xzyw(), vec4(1.0, 3.0, 2.0, 4.0));
    assert_eq!(v.xzzx(), vec4(1.0, 3.0, 3.0, 1.0));
    assert_eq!(v.xzzy(), vec4(1.0, 3.0, 3.0, 2.0));
    assert_eq!(v.xzzz(), vec4(1.0, 3.0, 3.0, 3.0));
    assert_eq!(v.xzzw(), vec4(1.0, 3.0, 3.0, 4.0));
    assert_eq!(v.xzwx(), vec4(1.0, 3.0, 4.0, 1.0));
    assert_eq!(v.xzwy(), vec4(1.0, 3.0, 4.0, 2.0));
    assert_eq!(v.xzwz(), vec4(1.0, 3.0, 4.0, 3.0));
    assert_eq!(v.xzww(), vec4(1.0, 3.0, 4.0, 4.0));
    assert_eq!(v.xwxx(), vec4(1.0, 4.0, 1.0, 1.0));
    assert_eq!(v.xwxy(), vec4(1.0, 4.0, 1.0, 2.0));
    assert_eq!(v.xwxz(), vec4(1.0, 4.0, 1.0, 3.0));
    assert_eq!(v.xwxw(), vec4(1.0, 4.0, 1.0, 4.0));
    assert_eq!(v.xwyx(), vec4(1.0, 4.0, 2.0, 1.0));
    assert_eq!(v.xwyy(), vec4(1.0, 4.0, 2.0, 2.0));
    assert_eq!(v.xwyz(), vec4(1.0, 4.0, 2.0, 3.0));
    assert_eq!(v.xwyw(), vec4(1.0, 4.0, 2.0, 4.0));
    assert_eq!(v.xwzx(), vec4(1.0, 4.0, 3.0, 1.0));
    assert_eq!(v.xwzy(), vec4(1.0, 4.0, 3.0, 2.0));
    assert_eq!(v.xwzz(), vec4(1.0, 4.0, 3.0, 3.0));
    assert_eq!(v.xwzw(), vec4(1.0, 4.0, 3.0, 4.0));
    assert_eq!(v.xwwx(), vec4(1.0, 4.0, 4.0, 1.0));
    assert_eq!(v.xwwy(), vec4(1.0, 4.0, 4.0, 2.0));
    assert_eq!(v.xwwz(), vec4(1.0, 4.0, 4.0, 3.0));
    assert_eq!(v.xwww(), vec4(1.0, 4.0, 4.0, 4.0));
    assert_eq!(v.yxxx(), vec4(2.0, 1.0, 1.0, 1.0));
    assert_eq!(v.yxxy(), vec4(2.0, 1.0, 1.0, 2.0));
    assert_eq!(v.yxxz(), vec4(2.0, 1.0, 1.0, 3.0));
    assert_eq!(v.yxxw(), vec4(2.0, 1.0, 1.0, 4.0));
    assert_eq!(v.yxyx(), vec4(2.0, 1.0, 2.0, 1.0));
    assert_eq!(v.yxyy(), vec4(2.0, 1.0, 2.0, 2.0));
    assert_eq!(v.yxyz(), vec4(2.0, 1.0, 2.0, 3.0));
    assert_eq!(v.yxyw(), vec4(2.0, 1.0, 2.0, 4.0));
    assert_eq!(v.yxzx(), vec4(2.0, 1.0, 3.0, 1.0));
    assert_eq!(v.yxzy(), vec4(2.0, 1.0, 3.0, 2.0));
    assert_eq!(v.yxzz(), vec4(2.0, 1.0, 3.0, 3.0));
    assert_eq!(v.yxzw(), vec4(2.0, 1.0, 3.0, 4.0));
    assert_eq!(v.yxwx(), vec4(2.0, 1.0, 4.0, 1.0));
    assert_eq!(v.yxwy(), vec4(2.0, 1.0, 4.0, 2.0));
    assert_eq!(v.yxwz(), vec4(2.0, 1.0, 4.0, 3.0));
    assert_eq!(v.yxww(), vec4(2.0, 1.0, 4.0, 4.0));
    assert_eq!(v.yyxx(), vec4(2.0, 2.0, 1.0, 1.0));
    assert_eq!(v.yyxy(), vec4(2.0, 2.0, 1.0, 2.0));
    assert_eq!(v.yyxz(), vec4(2.0, 2.0, 1.0, 3.0));
    assert_eq!(v.yyxw(), vec4(2.0, 2.0, 1.0, 4.0));
    assert_eq!(v.yyyx(), vec4(2.0, 2.0, 2.0, 1.0));
    assert_eq!(v.yyyy(), vec4(2.0, 2.0, 2.0, 2.0));
    assert_eq!(v.yyyz(), vec4(2.0, 2.0, 2.0, 3.0));
    assert_eq!(v.yyyw(), vec4(2.0, 2.0, 2.0, 4.0));
    assert_eq!(v.yyzx(), vec4(2.0, 2.0, 3.0, 1.0));
    assert_eq!(v.yyzy(), vec4(2.0, 2.0, 3.0, 2.0));
    assert_eq!(v.yyzz(), vec4(2.0, 2.0, 3.0, 3.0));
    assert_eq!(v.yyzw(), vec4(2.0, 2.0, 3.0, 4.0));
    assert_eq!(v.yywx(), vec4(2.0, 2.0, 4.0, 1.0));
    assert_eq!(v.yywy(), vec4(2.0, 2.0, 4.0, 2.0));
    assert_eq!(v.yywz(), vec4(2.0, 2.0, 4.0, 3.0));
    assert_eq!(v.yyww(), vec4(2.0, 2.0, 4.0, 4.0));
    assert_eq!(v.yzxx(), vec4(2.0, 3.0, 1.0, 1.0));
    assert_eq!(v.yzxy(), vec4(2.0, 3.0, 1.0, 2.0));
    assert_eq!(v.yzxz(), vec4(2.0, 3.0, 1.0, 3.0));
    assert_eq!(v.yzxw(), vec4(2.0, 3.0, 1.0, 4.0));
    assert_eq!(v.yzyx(), vec4(2.0, 3.0, 2.0, 1.0));
    assert_eq!(v.yzyy(), vec4(2.0, 3.0, 2.0, 2.0));
    assert_eq!(v.yzyz(), vec4(2.0, 3.0, 2.0, 3.0));
    assert_eq!(v.yzyw(), vec4(2.0, 3.0, 2.0, 4.0));
    assert_eq!(v.yzzx(), vec4(2.0, 3.0, 3.0, 1.0));
    assert_eq!(v.yzzy(), vec4(2.0, 3.0, 3.0, 2.0));
    assert_eq!(v.yzzz(), vec4(2.0, 3.0, 3.0, 3.0));
    assert_eq!(v.yzzw(), vec4(2.0, 3.0, 3.0, 4.0));
    assert_eq!(v.yzwx(), vec4(2.0, 3.0, 4.0, 1.0));
    assert_eq!(v.yzwy(), vec4(2.0, 3.0, 4.0, 2.0));
    assert_eq!(v.yzwz(), vec4(2.0, 3.0, 4.0, 3.0));
    assert_eq!(v.yzww(), vec4(2.0, 3.0, 4.0, 4.0));
    assert_eq!(v.ywxx(), vec4(2.0, 4.0, 1.0, 1.0));
    assert_eq!(v.ywxy(), vec4(2.0, 4.0, 1.0, 2.0));
    assert_eq!(v.ywxz(), vec4(2.0, 4.0, 1.0, 3.0));
    assert_eq!(v.ywxw(), vec4(2.0, 4.0, 1.0, 4.0));
    assert_eq!(v.ywyx(), vec4(2.0, 4.0, 2.0, 1.0));
    assert_eq!(v.ywyy(), vec4(2.0, 4.0, 2.0, 2.0));
    assert_eq!(v.ywyz(), vec4(2.0, 4.0, 2.0, 3.0));
    assert_eq!(v.ywyw(), vec4(2.0, 4.0, 2.0, 4.0));
    assert_eq!(v.ywzx(), vec4(2.0, 4.0, 3.0, 1.0));
    assert_eq!(v.ywzy(), vec4(2.0, 4.0, 3.0, 2.0));
    assert_eq!(v.ywzz(), vec4(2.0, 4.0, 3.0, 3.0));
    assert_eq!(v.ywzw(), vec4(2.0, 4.0, 3.0, 4.0));
    assert_eq!(v.ywwx(), vec4(2.0, 4.0, 4.0, 1.0));
    assert_eq!(v.ywwy(), vec4(2.0, 4.0, 4.0, 2.0));
    assert_eq!(v.ywwz(), vec4(2.0, 4.0, 4.0, 3.0));
    assert_eq!(v.ywww(), vec4(2.0, 4.0, 4.0, 4.0));
    assert_eq!(v.zxxx(), vec4(3.0, 1.0, 1.0, 1.0));
    assert_eq!(v.zxxy(), vec4(3.0, 1.0, 1.0, 2.0));
    assert_eq!(v.zxxz(), vec4(3.0, 1.0, 1.0, 3.0));
    assert_eq!(v.zxxw(), vec4(3.0, 1.0, 1.0, 4.0));
    assert_eq!(v.zxyx(), vec4(3.0, 1.0, 2.0, 1.0));
    assert_eq!(v.zxyy(), vec4(3.0, 1.0, 2.0, 2.0));
    assert_eq!(v.zxyz(), vec4(3.0, 1.0, 2.0, 3.0));
    assert_eq!(v.zxyw(), vec4(3.0, 1.0, 2.0, 4.0));
    assert_eq!(v.zxzx(), vec4(3.0, 1.0, 3.0, 1.0));
    assert_eq!(v.zxzy(), vec4(3.0, 1.0, 3.0, 2.0));
    assert_eq!(v.zxzz(), vec4(3.0, 1.0, 3.0, 3.0));
    assert_eq!(v.zxzw(), vec4(3.0, 1.0, 3.0, 4.0));
    assert_eq!(v.zxwx(), vec4(3.0, 1.0, 4.0, 1.0));
    assert_eq!(v.zxwy(), vec4(3.0, 1.0, 4.0, 2.0));
    assert_eq!(v.zxwz(), vec4(3.0, 1.0, 4.0, 3.0));
    assert_eq!(v.zxww(), vec4(3.0, 1.0, 4.0, 4.0));
    assert_eq!(v.zyxx(), vec4(3.0, 2.0, 1.0, 1.0));
    assert_eq!(v.zyxy(), vec4(3.0, 2.0, 1.0, 2.0));
    assert_eq!(v.zyxz(), vec4(3.0, 2.0, 1.0, 3.0));
    assert_eq!(v.zyxw(), vec4(3.0, 2.0, 1.0, 4.0));
    assert_eq!(v.zyyx(), vec4(3.0, 2.0, 2.0, 1.0));
    assert_eq!(v.zyyy(), vec4(3.0, 2.0, 2.0, 2.0));
    assert_eq!(v.zyyz(), vec4(3.0, 2.0, 2.0, 3.0));
    assert_eq!(v.zyyw(), vec4(3.0, 2.0, 2.0, 4.0));
    assert_eq!(v.zyzx(), vec4(3.0, 2.0, 3.0, 1.0));
    assert_eq!(v.zyzy(), vec4(3.0, 2.0, 3.0, 2.0));
    assert_eq!(v.zyzz(), vec4(3.0, 2.0, 3.0, 3.0));
    assert_eq!(v.zyzw(), vec4(3.0, 2.0, 3.0, 4.0));
    assert_eq!(v.zywx(), vec4(3.0, 2.0, 4.0, 1.0));
    assert_eq!(v.zywy(), vec4(3.0, 2.0, 4.0, 2.0));
    assert_eq!(v.zywz(), vec4(3.0, 2.0, 4.0, 3.0));
    assert_eq!(v.zyww(), vec4(3.0, 2.0, 4.0, 4.0));
    assert_eq!(v.zzxx(), vec4(3.0, 3.0, 1.0, 1.0));
    assert_eq!(v.zzxy(), vec4(3.0, 3.0, 1.0, 2.0));
    assert_eq!(v.zzxz(), vec4(3.0, 3.0, 1.0, 3.0));
    assert_eq!(v.zzxw(), vec4(3.0, 3.0, 1.0, 4.0));
    assert_eq!(v.zzyx(), vec4(3.0, 3.0, 2.0, 1.0));
    assert_eq!(v.zzyy(), vec4(3.0, 3.0, 2.0, 2.0));
    assert_eq!(v.zzyz(), vec4(3.0, 3.0, 2.0, 3.0));
    assert_eq!(v.zzyw(), vec4(3.0, 3.0, 2.0, 4.0));
    assert_eq!(v.zzzx(), vec4(3.0, 3.0, 3.0, 1.0));
    assert_eq!(v.zzzy(), vec4(3.0, 3.0, 3.0, 2.0));
    assert_eq!(v.zzzz(), vec4(3.0, 3.0, 3.0, 3.0));
    assert_eq!(v.zzzw(), vec4(3.0, 3.0, 3.0, 4.0));
    assert_eq!(v.zzwx(), vec4(3.0, 3.0, 4.0, 1.0));
    assert_eq!(v.zzwy(), vec4(3.0, 3.0, 4.0, 2.0));
    assert_eq!(v.zzwz(), vec4(3.0, 3.0, 4.0, 3.0));
    assert_eq!(v.zzww(), vec4(3.0, 3.0, 4.0, 4.0));
    assert_eq!(v.zwxx(), vec4(3.0, 4.0, 1.0, 1.0));
    assert_eq!(v.zwxy(), vec4(3.0, 4.0, 1.0, 2.0));
    assert_eq!(v.zwxz(), vec4(3.0, 4.0, 1.0, 3.0));
    assert_eq!(v.zwxw(), vec4(3.0, 4.0, 1.0, 4.0));
    assert_eq!(v.zwyx(), vec4(3.0, 4.0, 2.0, 1.0));
    assert_eq!(v.zwyy(), vec4(3.0, 4.0, 2.0, 2.0));
    assert_eq!(v.zwyz(), vec4(3.0, 4.0, 2.0, 3.0));
    assert_eq!(v.zwyw(), vec4(3.0, 4.0, 2.0, 4.0));
    assert_eq!(v.zwzx(), vec4(3.0, 4.0, 3.0, 1.0));
    assert_eq!(v.zwzy(), vec4(3.0, 4.0, 3.0, 2.0));
    assert_eq!(v.zwzz(), vec4(3.0, 4.0, 3.0, 3.0));
    assert_eq!(v.zwzw(), vec4(3.0, 4.0, 3.0, 4.0));
    assert_eq!(v.zwwx(), vec4(3.0, 4.0, 4.0, 1.0));
    assert_eq!(v.zwwy(), vec4(3.0, 4.0, 4.0, 2.0));
    assert_eq!(v.zwwz(), vec4(3.0, 4.0, 4.0, 3.0));
    assert_eq!(v.zwww(), vec4(3.0, 4.0, 4.0, 4.0));
    assert_eq!(v.wxxx(), vec4(4.0, 1.0, 1.0, 1.0));
    assert_eq!(v.wxxy(), vec4(4.0, 1.0, 1.0, 2.0));
    assert_eq!(v.wxxz(), vec4(4.0, 1.0, 1.0, 3.0));
    assert_eq!(v.wxxw(), vec4(4.0, 1.0, 1.0, 4.0));
    assert_eq!(v.wxyx(), vec4(4.0, 1.0, 2.0, 1.0));
    assert_eq!(v.wxyy(), vec4(4.0, 1.0, 2.0, 2.0));
    assert_eq!(v.wxyz(), vec4(4.0, 1.0, 2.0, 3.0));
    assert_eq!(v.wxyw(), vec4(4.0, 1.0, 2.0, 4.0));
    assert_eq!(v.wxzx(), vec4(4.0, 1.0, 3.0, 1.0));
    assert_eq!(v.wxzy(), vec4(4.0, 1.0, 3.0, 2.0));
    assert_eq!(v.wxzz(), vec4(4.0, 1.0, 3.0, 3.0));
    assert_eq!(v.wxzw(), vec4(4.0, 1.0, 3.0, 4.0));
    assert_eq!(v.wxwx(), vec4(4.0, 1.0, 4.0, 1.0));
    assert_eq!(v.wxwy(), vec4(4.0, 1.0, 4.0, 2.0));
    assert_eq!(v.wxwz(), vec4(4.0, 1.0, 4.0, 3.0));
    assert_eq!(v.wxww(), vec4(4.0, 1.0, 4.0, 4.0));
    assert_eq!(v.wyxx(), vec4(4.0, 2.0, 1.0, 1.0));
    assert_eq!(v.wyxy(), vec4(4.0, 2.0, 1.0, 2.0));
    assert_eq!(v.wyxz(), vec4(4.0, 2.0, 1.0, 3.0));
    assert_eq!(v.wyxw(), vec4(4.0, 2.0, 1.0, 4.0));
    assert_eq!(v.wyyx(), vec4(4.0, 2.0, 2.0, 1.0));
    assert_eq!(v.wyyy(), vec4(4.0, 2.0, 2.0, 2.0));
    assert_eq!(v.wyyz(), vec4(4.0, 2.0, 2.0, 3.0));
    assert_eq!(v.wyyw(), vec4(4.0, 2.0, 2.0, 4.0));
    assert_eq!(v.wyzx(), vec4(4.0, 2.0, 3.0, 1.0));
    assert_eq!(v.wyzy(), vec4(4.0, 2.0, 3.0, 2.0));
    assert_eq!(v.wyzz(), vec4(4.0, 2.0, 3.0, 3.0));
    assert_eq!(v.wyzw(), vec4(4.0, 2.0, 3.0, 4.0));
    assert_eq!(v.wywx(), vec4(4.0, 2.0, 4.0, 1.0));
    assert_eq!(v.wywy(), vec4(4.0, 2.0, 4.0, 2.0));
    assert_eq!(v.wywz(), vec4(4.0, 2.0, 4.0, 3.0));
    assert_eq!(v.wyww(), vec4(4.0, 2.0, 4.0, 4.0));
    assert_eq!(v.wzxx(), vec4(4.0, 3.0, 1.0, 1.0));
    assert_eq!(v.wzxy(), vec4(4.0, 3.0, 1.0, 2.0));
    assert_eq!(v.wzxz(), vec4(4.0, 3.0, 1.0, 3.0));
    assert_eq!(v.wzxw(), vec4(4.0, 3.0, 1.0, 4.0));
    assert_eq!(v.wzyx(), vec4(4.0, 3.0, 2.0, 1.0));
    assert_eq!(v.wzyy(), vec4(4.0, 3.0, 2.0, 2.0));
    assert_eq!(v.wzyz(), vec4(4.0, 3.0, 2.0, 3.0));
    assert_eq!(v.wzyw(), vec4(4.0, 3.0, 2.0, 4.0));
    assert_eq!(v.wzzx(), vec4(4.0, 3.0, 3.0, 1.0));
    assert_eq!(v.wzzy(), vec4(4.0, 3.0, 3.0, 2.0));
    assert_eq!(v.wzzz(), vec4(4.0, 3.0, 3.0, 3.0));
    assert_eq!(v.wzzw(), vec4(4.0, 3.0, 3.0, 4.0));
    assert_eq!(v.wzwx(), vec4(4.0, 3.0, 4.0, 1.0));
    assert_eq!(v.wzwy(), vec4(4.0, 3.0, 4.0, 2.0));
    assert_eq!(v.wzwz(), vec4(4.0, 3.0, 4.0, 3.0));
    assert_eq!(v.wzww(), vec4(4.0, 3.0, 4.0, 4.0));
    assert_eq!(v.wwxx(), vec4(4.0, 4.0, 1.0, 1.0));
    assert_eq!(v.wwxy(), vec4(4.0, 4.0, 1.0, 2.0));
    assert_eq!(v.wwxz(), vec4(4.0, 4.0, 1.0, 3.0));
    assert_eq!(v.wwxw(), vec4(4.0, 4.0, 1.0, 4.0));
    assert_eq!(v.wwyx(), vec4(4.0, 4.0, 2.0, 1.0));
    assert_eq!(v.wwyy(), vec4(4.0, 4.0, 2.0, 2.0));
    assert_eq!(v.wwyz(), vec4(4.0, 4.0, 2.0, 3.0));
    assert_eq!(v.wwyw(), vec4(4.0, 4.0, 2.0, 4.0));
    assert_eq!(v.wwzx(), vec4(4.0, 4.0, 3.0, 1.0));
    assert_eq!(v.wwzy(), vec4(4.0, 4.0, 3.0, 2.0));
    assert_eq!(v.wwzz(), vec4(4.0, 4.0, 3.0, 3.0));
    assert_eq!(v.wwzw(), vec4(4.0, 4.0, 3.0, 4.0));
    assert_eq!(v.wwwx(), vec4(4.0, 4.0, 4.0, 1.0));
    assert_eq!(v.wwwy(), vec4(4.0, 4.0, 4.0, 2.0));
    assert_eq!(v.wwwz(), vec4(4.0, 4.0, 4.0, 3.0));
    assert_eq!(v.wwww(), vec4(4.0, 4.0, 4.0, 4.0));
    assert_eq!(v.xxx(), vec3(1.0, 1.0, 1.0));
    assert_eq!(v.xxy(), vec3(1.0, 1.0, 2.0));
    assert_eq!(v.xxz(), vec3(1.0, 1.0, 3.0));
    assert_eq!(v.xxw(), vec3(1.0, 1.0, 4.0));
    assert_eq!(v.xyx(), vec3(1.0, 2.0, 1.0));
    assert_eq!(v.xyy(), vec3(1.0, 2.0, 2.0));
    assert_eq!(v.xyz(), vec3(1.0, 2.0, 3.0));
    assert_eq!(v.xyw(), vec3(1.0, 2.0, 4.0));
    assert_eq!(v.xzx(), vec3(1.0, 3.0, 1.0));
    assert_eq!(v.xzy(), vec3(1.0, 3.0, 2.0));
    assert_eq!(v.xzz(), vec3(1.0, 3.0, 3.0));
    assert_eq!(v.xzw(), vec3(1.0, 3.0, 4.0));
    assert_eq!(v.xwx(), vec3(1.0, 4.0, 1.0));
    assert_eq!(v.xwy(), vec3(1.0, 4.0, 2.0));
    assert_eq!(v.xwz(), vec3(1.0, 4.0, 3.0));
    assert_eq!(v.xww(), vec3(1.0, 4.0, 4.0));
    assert_eq!(v.yxx(), vec3(2.0, 1.0, 1.0));
    assert_eq!(v.yxy(), vec3(2.0, 1.0, 2.0));
    assert_eq!(v.yxz(), vec3(2.0, 1.0, 3.0));
    assert_eq!(v.yxw(), vec3(2.0, 1.0, 4.0));
    assert_eq!(v.yyx(), vec3(2.0, 2.0, 1.0));
    assert_eq!(v.yyy(), vec3(2.0, 2.0, 2.0));
    assert_eq!(v.yyz(), vec3(2.0, 2.0, 3.0));
    assert_eq!(v.yyw(), vec3(2.0, 2.0, 4.0));
    assert_eq!(v.yzx(), vec3(2.0, 3.0, 1.0));
    assert_eq!(v.yzy(), vec3(2.0, 3.0, 2.0));
    assert_eq!(v.yzz(), vec3(2.0, 3.0, 3.0));
    assert_eq!(v.yzw(), vec3(2.0, 3.0, 4.0));
    assert_eq!(v.ywx(), vec3(2.0, 4.0, 1.0));
    assert_eq!(v.ywy(), vec3(2.0, 4.0, 2.0));
    assert_eq!(v.ywz(), vec3(2.0, 4.0, 3.0));
    assert_eq!(v.yww(), vec3(2.0, 4.0, 4.0));
    assert_eq!(v.zxx(), vec3(3.0, 1.0, 1.0));
    assert_eq!(v.zxy(), vec3(3.0, 1.0, 2.0));
    assert_eq!(v.zxz(), vec3(3.0, 1.0, 3.0));
    assert_eq!(v.zxw(), vec3(3.0, 1.0, 4.0));
    assert_eq!(v.zyx(), vec3(3.0, 2.0, 1.0));
    assert_eq!(v.zyy(), vec3(3.0, 2.0, 2.0));
    assert_eq!(v.zyz(), vec3(3.0, 2.0, 3.0));
    assert_eq!(v.zyw(), vec3(3.0, 2.0, 4.0));
    assert_eq!(v.zzx(), vec3(3.0, 3.0, 1.0));
    assert_eq!(v.zzy(), vec3(3.0, 3.0, 2.0));
    assert_eq!(v.zzz(), vec3(3.0, 3.0, 3.0));
    assert_eq!(v.zzw(), vec3(3.0, 3.0, 4.0));
    assert_eq!(v.zwx(), vec3(3.0, 4.0, 1.0));
    assert_eq!(v.zwy(), vec3(3.0, 4.0, 2.0));
    assert_eq!(v.zwz(), vec3(3.0, 4.0, 3.0));
    assert_eq!(v.zww(), vec3(3.0, 4.0, 4.0));
    assert_eq!(v.wxx(), vec3(4.0, 1.0, 1.0));
    assert_eq!(v.wxy(), vec3(4.0, 1.0, 2.0));
    assert_eq!(v.wxz(), vec3(4.0, 1.0, 3.0));
    assert_eq!(v.wxw(), vec3(4.0, 1.0, 4.0));
    assert_eq!(v.wyx(), vec3(4.0, 2.0, 1.0));
    assert_eq!(v.wyy(), vec3(4.0, 2.0, 2.0));
    assert_eq!(v.wyz(), vec3(4.0, 2.0, 3.0));
    assert_eq!(v.wyw(), vec3(4.0, 2.0, 4.0));
    assert_eq!(v.wzx(), vec3(4.0, 3.0, 1.0));
    assert_eq!(v.wzy(), vec3(4.0, 3.0, 2.0));
    assert_eq!(v.wzz(), vec3(4.0, 3.0, 3.0));
    assert_eq!(v.wzw(), vec3(4.0, 3.0, 4.0));
    assert_eq!(v.wwx(), vec3(4.0, 4.0, 1.0));
    assert_eq!(v.wwy(), vec3(4.0, 4.0, 2.0));
    assert_eq!(v.wwz(), vec3(4.0, 4.0, 3.0));
    assert_eq!(v.www(), vec3(4.0, 4.0, 4.0));
    assert_eq!(v.xx(), vec2(1.0, 1.0));
    assert_eq!(v.xy(), vec2(1.0, 2.0));
    assert_eq!(v.xz(), vec2(1.0, 3.0));
    assert_eq!(v.xw(), vec2(1.0, 4.0));
    assert_eq!(v.yx(), vec2(2.0, 1.0));
    assert_eq!(v.yy(), vec2(2.0, 2.0));
    assert_eq!(v.yz(), vec2(2.0, 3.0));
    assert_eq!(v.yw(), vec2(2.0, 4.0));
    assert_eq!(v.zx(), vec2(3.0, 1.0));
    assert_eq!(v.zy(), vec2(3.0, 2.0));
    assert_eq!(v.zz(), vec2(3.0, 3.0));
    assert_eq!(v.zw(), vec2(3.0, 4.0));
    assert_eq!(v.wx(), vec2(4.0, 1.0));
    assert_eq!(v.wy(), vec2(4.0, 2.0));
    assert_eq!(v.wz(), vec2(4.0, 3.0));
    assert_eq!(v.ww(), vec2(4.0, 4.0));
}

#[test]
fn test_vec3a_swizzles() {
    let v = vec3a(1.0, 2.0, 3.0);
    assert_eq!(v.xxxx(), vec4(1.0, 1.0, 1.0, 1.0));
    assert_eq!(v.xxxy(), vec4(1.0, 1.0, 1.0, 2.0));
    assert_eq!(v.xxxz(), vec4(1.0, 1.0, 1.0, 3.0));
    assert_eq!(v.xxyx(), vec4(1.0, 1.0, 2.0, 1.0));
    assert_eq!(v.xxyy(), vec4(1.0, 1.0, 2.0, 2.0));
    assert_eq!(v.xxyz(), vec4(1.0, 1.0, 2.0, 3.0));
    assert_eq!(v.xxzx(), vec4(1.0, 1.0, 3.0, 1.0));
    assert_eq!(v.xxzy(), vec4(1.0, 1.0, 3.0, 2.0));
    assert_eq!(v.xxzz(), vec4(1.0, 1.0, 3.0, 3.0));
    assert_eq!(v.xyxx(), vec4(1.0, 2.0, 1.0, 1.0));
    assert_eq!(v.xyxy(), vec4(1.0, 2.0, 1.0, 2.0));
    assert_eq!(v.xyxz(), vec4(1.0, 2.0, 1.0, 3.0));
    assert_eq!(v.xyyx(), vec4(1.0, 2.0, 2.0, 1.0));
    assert_eq!(v.xyyy(), vec4(1.0, 2.0, 2.0, 2.0));
    assert_eq!(v.xyyz(), vec4(1.0, 2.0, 2.0, 3.0));
    assert_eq!(v.xyzx(), vec4(1.0, 2.0, 3.0, 1.0));
    assert_eq!(v.xyzy(), vec4(1.0, 2.0, 3.0, 2.0));
    assert_eq!(v.xyzz(), vec4(1.0, 2.0, 3.0, 3.0));
    assert_eq!(v.xzxx(), vec4(1.0, 3.0, 1.0, 1.0));
    assert_eq!(v.xzxy(), vec4(1.0, 3.0, 1.0, 2.0));
    assert_eq!(v.xzxz(), vec4(1.0, 3.0, 1.0, 3.0));
    assert_eq!(v.xzyx(), vec4(1.0, 3.0, 2.0, 1.0));
    assert_eq!(v.xzyy(), vec4(1.0, 3.0, 2.0, 2.0));
    assert_eq!(v.xzyz(), vec4(1.0, 3.0, 2.0, 3.0));
    assert_eq!(v.xzzx(), vec4(1.0, 3.0, 3.0, 1.0));
    assert_eq!(v.xzzy(), vec4(1.0, 3.0, 3.0, 2.0));
    assert_eq!(v.xzzz(), vec4(1.0, 3.0, 3.0, 3.0));
    assert_eq!(v.yxxx(), vec4(2.0, 1.0, 1.0, 1.0));
    assert_eq!(v.yxxy(), vec4(2.0, 1.0, 1.0, 2.0));
    assert_eq!(v.yxxz(), vec4(2.0, 1.0, 1.0, 3.0));
    assert_eq!(v.yxyx(), vec4(2.0, 1.0, 2.0, 1.0));
    assert_eq!(v.yxyy(), vec4(2.0, 1.0, 2.0, 2.0));
    assert_eq!(v.yxyz(), vec4(2.0, 1.0, 2.0, 3.0));
    assert_eq!(v.yxzx(), vec4(2.0, 1.0, 3.0, 1.0));
    assert_eq!(v.yxzy(), vec4(2.0, 1.0, 3.0, 2.0));
    assert_eq!(v.yxzz(), vec4(2.0, 1.0, 3.0, 3.0));
    assert_eq!(v.yyxx(), vec4(2.0, 2.0, 1.0, 1.0));
    assert_eq!(v.yyxy(), vec4(2.0, 2.0, 1.0, 2.0));
    assert_eq!(v.yyxz(), vec4(2.0, 2.0, 1.0, 3.0));
    assert_eq!(v.yyyx(), vec4(2.0, 2.0, 2.0, 1.0));
    assert_eq!(v.yyyy(), vec4(2.0, 2.0, 2.0, 2.0));
    assert_eq!(v.yyyz(), vec4(2.0, 2.0, 2.0, 3.0));
    assert_eq!(v.yyzx(), vec4(2.0, 2.0, 3.0, 1.0));
    assert_eq!(v.yyzy(), vec4(2.0, 2.0, 3.0, 2.0));
    assert_eq!(v.yyzz(), vec4(2.0, 2.0, 3.0, 3.0));
    assert_eq!(v.yzxx(), vec4(2.0, 3.0, 1.0, 1.0));
    assert_eq!(v.yzxy(), vec4(2.0, 3.0, 1.0, 2.0));
    assert_eq!(v.yzxz(), vec4(2.0, 3.0, 1.0, 3.0));
    assert_eq!(v.yzyx(), vec4(2.0, 3.0, 2.0, 1.0));
    assert_eq!(v.yzyy(), vec4(2.0, 3.0, 2.0, 2.0));
    assert_eq!(v.yzyz(), vec4(2.0, 3.0, 2.0, 3.0));
    assert_eq!(v.yzzx(), vec4(2.0, 3.0, 3.0, 1.0));
    assert_eq!(v.yzzy(), vec4(2.0, 3.0, 3.0, 2.0));
    assert_eq!(v.yzzz(), vec4(2.0, 3.0, 3.0, 3.0));
    assert_eq!(v.zxxx(), vec4(3.0, 1.0, 1.0, 1.0));
    assert_eq!(v.zxxy(), vec4(3.0, 1.0, 1.0, 2.0));
    assert_eq!(v.zxxz(), vec4(3.0, 1.0, 1.0, 3.0));
    assert_eq!(v.zxyx(), vec4(3.0, 1.0, 2.0, 1.0));
    assert_eq!(v.zxyy(), vec4(3.0, 1.0, 2.0, 2.0));
    assert_eq!(v.zxyz(), vec4(3.0, 1.0, 2.0, 3.0));
    assert_eq!(v.zxzx(), vec4(3.0, 1.0, 3.0, 1.0));
    assert_eq!(v.zxzy(), vec4(3.0, 1.0, 3.0, 2.0));
    assert_eq!(v.zxzz(), vec4(3.0, 1.0, 3.0, 3.0));
    assert_eq!(v.zyxx(), vec4(3.0, 2.0, 1.0, 1.0));
    assert_eq!(v.zyxy(), vec4(3.0, 2.0, 1.0, 2.0));
    assert_eq!(v.zyxz(), vec4(3.0, 2.0, 1.0, 3.0));
    assert_eq!(v.zyyx(), vec4(3.0, 2.0, 2.0, 1.0));
    assert_eq!(v.zyyy(), vec4(3.0, 2.0, 2.0, 2.0));
    assert_eq!(v.zyyz(), vec4(3.0, 2.0, 2.0, 3.0));
    assert_eq!(v.zyzx(), vec4(3.0, 2.0, 3.0, 1.0));
    assert_eq!(v.zyzy(), vec4(3.0, 2.0, 3.0, 2.0));
    assert_eq!(v.zyzz(), vec4(3.0, 2.0, 3.0, 3.0));
    assert_eq!(v.zzxx(), vec4(3.0, 3.0, 1.0, 1.0));
    assert_eq!(v.zzxy(), vec4(3.0, 3.0, 1.0, 2.0));
    assert_eq!(v.zzxz(), vec4(3.0, 3.0, 1.0, 3.0));
    assert_eq!(v.zzyx(), vec4(3.0, 3.0, 2.0, 1.0));
    assert_eq!(v.zzyy(), vec4(3.0, 3.0, 2.0, 2.0));
    assert_eq!(v.zzyz(), vec4(3.0, 3.0, 2.0, 3.0));
    assert_eq!(v.zzzx(), vec4(3.0, 3.0, 3.0, 1.0));
    assert_eq!(v.zzzy(), vec4(3.0, 3.0, 3.0, 2.0));
    assert_eq!(v.zzzz(), vec4(3.0, 3.0, 3.0, 3.0));
    assert_eq!(v.xxx(), vec3a(1.0, 1.0, 1.0));
    assert_eq!(v.xxy(), vec3a(1.0, 1.0, 2.0));
    assert_eq!(v.xxz(), vec3a(1.0, 1.0, 3.0));
    assert_eq!(v.xyx(), vec3a(1.0, 2.0, 1.0));
    assert_eq!(v.xyy(), vec3a(1.0, 2.0, 2.0));
    assert_eq!(v.xzx(), vec3a(1.0, 3.0, 1.0));
    assert_eq!(v.xzy(), vec3a(1.0, 3.0, 2.0));
    assert_eq!(v.xzz(), vec3a(1.0, 3.0, 3.0));
    assert_eq!(v.yxx(), vec3a(2.0, 1.0, 1.0));
    assert_eq!(v.yxy(), vec3a(2.0, 1.0, 2.0));
    assert_eq!(v.yxz(), vec3a(2.0, 1.0, 3.0));
    assert_eq!(v.yyx(), vec3a(2.0, 2.0, 1.0));
    assert_eq!(v.yyy(), vec3a(2.0, 2.0, 2.0));
    assert_eq!(v.yyz(), vec3a(2.0, 2.0, 3.0));
    assert_eq!(v.yzx(), vec3a(2.0, 3.0, 1.0));
    assert_eq!(v.yzy(), vec3a(2.0, 3.0, 2.0));
    assert_eq!(v.yzz(), vec3a(2.0, 3.0, 3.0));
    assert_eq!(v.zxx(), vec3a(3.0, 1.0, 1.0));
    assert_eq!(v.zxy(), vec3a(3.0, 1.0, 2.0));
    assert_eq!(v.zxz(), vec3a(3.0, 1.0, 3.0));
    assert_eq!(v.zyx(), vec3a(3.0, 2.0, 1.0));
    assert_eq!(v.zyy(), vec3a(3.0, 2.0, 2.0));
    assert_eq!(v.zyz(), vec3a(3.0, 2.0, 3.0));
    assert_eq!(v.zzx(), vec3a(3.0, 3.0, 1.0));
    assert_eq!(v.zzy(), vec3a(3.0, 3.0, 2.0));
    assert_eq!(v.zzz(), vec3a(3.0, 3.0, 3.0));
    assert_eq!(v.xx(), vec2(1.0, 1.0));
    assert_eq!(v.xy(), vec2(1.0, 2.0));
    assert_eq!(v.xz(), vec2(1.0, 3.0));
    assert_eq!(v.yx(), vec2(2.0, 1.0));
    assert_eq!(v.yy(), vec2(2.0, 2.0));
    assert_eq!(v.yz(), vec2(2.0, 3.0));
    assert_eq!(v.zx(), vec2(3.0, 1.0));
    assert_eq!(v.zy(), vec2(3.0, 2.0));
    assert_eq!(v.zz(), vec2(3.0, 3.0));
}

#[test]
fn test_vec3_swizzles() {
    let v = vec3(1.0, 2.0, 3.0);
    assert_eq!(v.xxxx(), vec4(1.0, 1.0, 1.0, 1.0));
    assert_eq!(v.xxxy(), vec4(1.0, 1.0, 1.0, 2.0));
    assert_eq!(v.xxxz(), vec4(1.0, 1.0, 1.0, 3.0));
    assert_eq!(v.xxyx(), vec4(1.0, 1.0, 2.0, 1.0));
    assert_eq!(v.xxyy(), vec4(1.0, 1.0, 2.0, 2.0));
    assert_eq!(v.xxyz(), vec4(1.0, 1.0, 2.0, 3.0));
    assert_eq!(v.xxzx(), vec4(1.0, 1.0, 3.0, 1.0));
    assert_eq!(v.xxzy(), vec4(1.0, 1.0, 3.0, 2.0));
    assert_eq!(v.xxzz(), vec4(1.0, 1.0, 3.0, 3.0));
    assert_eq!(v.xyxx(), vec4(1.0, 2.0, 1.0, 1.0));
    assert_eq!(v.xyxy(), vec4(1.0, 2.0, 1.0, 2.0));
    assert_eq!(v.xyxz(), vec4(1.0, 2.0, 1.0, 3.0));
    assert_eq!(v.xyyx(), vec4(1.0, 2.0, 2.0, 1.0));
    assert_eq!(v.xyyy(), vec4(1.0, 2.0, 2.0, 2.0));
    assert_eq!(v.xyyz(), vec4(1.0, 2.0, 2.0, 3.0));
    assert_eq!(v.xyzx(), vec4(1.0, 2.0, 3.0, 1.0));
    assert_eq!(v.xyzy(), vec4(1.0, 2.0, 3.0, 2.0));
    assert_eq!(v.xyzz(), vec4(1.0, 2.0, 3.0, 3.0));
    assert_eq!(v.xzxx(), vec4(1.0, 3.0, 1.0, 1.0));
    assert_eq!(v.xzxy(), vec4(1.0, 3.0, 1.0, 2.0));
    assert_eq!(v.xzxz(), vec4(1.0, 3.0, 1.0, 3.0));
    assert_eq!(v.xzyx(), vec4(1.0, 3.0, 2.0, 1.0));
    assert_eq!(v.xzyy(), vec4(1.0, 3.0, 2.0, 2.0));
    assert_eq!(v.xzyz(), vec4(1.0, 3.0, 2.0, 3.0));
    assert_eq!(v.xzzx(), vec4(1.0, 3.0, 3.0, 1.0));
    assert_eq!(v.xzzy(), vec4(1.0, 3.0, 3.0, 2.0));
    assert_eq!(v.xzzz(), vec4(1.0, 3.0, 3.0, 3.0));
    assert_eq!(v.yxxx(), vec4(2.0, 1.0, 1.0, 1.0));
    assert_eq!(v.yxxy(), vec4(2.0, 1.0, 1.0, 2.0));
    assert_eq!(v.yxxz(), vec4(2.0, 1.0, 1.0, 3.0));
    assert_eq!(v.yxyx(), vec4(2.0, 1.0, 2.0, 1.0));
    assert_eq!(v.yxyy(), vec4(2.0, 1.0, 2.0, 2.0));
    assert_eq!(v.yxyz(), vec4(2.0, 1.0, 2.0, 3.0));
    assert_eq!(v.yxzx(), vec4(2.0, 1.0, 3.0, 1.0));
    assert_eq!(v.yxzy(), vec4(2.0, 1.0, 3.0, 2.0));
    assert_eq!(v.yxzz(), vec4(2.0, 1.0, 3.0, 3.0));
    assert_eq!(v.yyxx(), vec4(2.0, 2.0, 1.0, 1.0));
    assert_eq!(v.yyxy(), vec4(2.0, 2.0, 1.0, 2.0));
    assert_eq!(v.yyxz(), vec4(2.0, 2.0, 1.0, 3.0));
    assert_eq!(v.yyyx(), vec4(2.0, 2.0, 2.0, 1.0));
    assert_eq!(v.yyyy(), vec4(2.0, 2.0, 2.0, 2.0));
    assert_eq!(v.yyyz(), vec4(2.0, 2.0, 2.0, 3.0));
    assert_eq!(v.yyzx(), vec4(2.0, 2.0, 3.0, 1.0));
    assert_eq!(v.yyzy(), vec4(2.0, 2.0, 3.0, 2.0));
    assert_eq!(v.yyzz(), vec4(2.0, 2.0, 3.0, 3.0));
    assert_eq!(v.yzxx(), vec4(2.0, 3.0, 1.0, 1.0));
    assert_eq!(v.yzxy(), vec4(2.0, 3.0, 1.0, 2.0));
    assert_eq!(v.yzxz(), vec4(2.0, 3.0, 1.0, 3.0));
    assert_eq!(v.yzyx(), vec4(2.0, 3.0, 2.0, 1.0));
    assert_eq!(v.yzyy(), vec4(2.0, 3.0, 2.0, 2.0));
    assert_eq!(v.yzyz(), vec4(2.0, 3.0, 2.0, 3.0));
    assert_eq!(v.yzzx(), vec4(2.0, 3.0, 3.0, 1.0));
    assert_eq!(v.yzzy(), vec4(2.0, 3.0, 3.0, 2.0));
    assert_eq!(v.yzzz(), vec4(2.0, 3.0, 3.0, 3.0));
    assert_eq!(v.zxxx(), vec4(3.0, 1.0, 1.0, 1.0));
    assert_eq!(v.zxxy(), vec4(3.0, 1.0, 1.0, 2.0));
    assert_eq!(v.zxxz(), vec4(3.0, 1.0, 1.0, 3.0));
    assert_eq!(v.zxyx(), vec4(3.0, 1.0, 2.0, 1.0));
    assert_eq!(v.zxyy(), vec4(3.0, 1.0, 2.0, 2.0));
    assert_eq!(v.zxyz(), vec4(3.0, 1.0, 2.0, 3.0));
    assert_eq!(v.zxzx(), vec4(3.0, 1.0, 3.0, 1.0));
    assert_eq!(v.zxzy(), vec4(3.0, 1.0, 3.0, 2.0));
    assert_eq!(v.zxzz(), vec4(3.0, 1.0, 3.0, 3.0));
    assert_eq!(v.zyxx(), vec4(3.0, 2.0, 1.0, 1.0));
    assert_eq!(v.zyxy(), vec4(3.0, 2.0, 1.0, 2.0));
    assert_eq!(v.zyxz(), vec4(3.0, 2.0, 1.0, 3.0));
    assert_eq!(v.zyyx(), vec4(3.0, 2.0, 2.0, 1.0));
    assert_eq!(v.zyyy(), vec4(3.0, 2.0, 2.0, 2.0));
    assert_eq!(v.zyyz(), vec4(3.0, 2.0, 2.0, 3.0));
    assert_eq!(v.zyzx(), vec4(3.0, 2.0, 3.0, 1.0));
    assert_eq!(v.zyzy(), vec4(3.0, 2.0, 3.0, 2.0));
    assert_eq!(v.zyzz(), vec4(3.0, 2.0, 3.0, 3.0));
    assert_eq!(v.zzxx(), vec4(3.0, 3.0, 1.0, 1.0));
    assert_eq!(v.zzxy(), vec4(3.0, 3.0, 1.0, 2.0));
    assert_eq!(v.zzxz(), vec4(3.0, 3.0, 1.0, 3.0));
    assert_eq!(v.zzyx(), vec4(3.0, 3.0, 2.0, 1.0));
    assert_eq!(v.zzyy(), vec4(3.0, 3.0, 2.0, 2.0));
    assert_eq!(v.zzyz(), vec4(3.0, 3.0, 2.0, 3.0));
    assert_eq!(v.zzzx(), vec4(3.0, 3.0, 3.0, 1.0));
    assert_eq!(v.zzzy(), vec4(3.0, 3.0, 3.0, 2.0));
    assert_eq!(v.zzzz(), vec4(3.0, 3.0, 3.0, 3.0));
    assert_eq!(v.xxx(), vec3(1.0, 1.0, 1.0));
    assert_eq!(v.xxy(), vec3(1.0, 1.0, 2.0));
    assert_eq!(v.xxz(), vec3(1.0, 1.0, 3.0));
    assert_eq!(v.xyx(), vec3(1.0, 2.0, 1.0));
    assert_eq!(v.xyy(), vec3(1.0, 2.0, 2.0));
    assert_eq!(v.xzx(), vec3(1.0, 3.0, 1.0));
    assert_eq!(v.xzy(), vec3(1.0, 3.0, 2.0));
    assert_eq!(v.xzz(), vec3(1.0, 3.0, 3.0));
    assert_eq!(v.yxx(), vec3(2.0, 1.0, 1.0));
    assert_eq!(v.yxy(), vec3(2.0, 1.0, 2.0));
    assert_eq!(v.yxz(), vec3(2.0, 1.0, 3.0));
    assert_eq!(v.yyx(), vec3(2.0, 2.0, 1.0));
    assert_eq!(v.yyy(), vec3(2.0, 2.0, 2.0));
    assert_eq!(v.yyz(), vec3(2.0, 2.0, 3.0));
    assert_eq!(v.yzx(), vec3(2.0, 3.0, 1.0));
    assert_eq!(v.yzy(), vec3(2.0, 3.0, 2.0));
    assert_eq!(v.yzz(), vec3(2.0, 3.0, 3.0));
    assert_eq!(v.zxx(), vec3(3.0, 1.0, 1.0));
    assert_eq!(v.zxy(), vec3(3.0, 1.0, 2.0));
    assert_eq!(v.zxz(), vec3(3.0, 1.0, 3.0));
    assert_eq!(v.zyx(), vec3(3.0, 2.0, 1.0));
    assert_eq!(v.zyy(), vec3(3.0, 2.0, 2.0));
    assert_eq!(v.zyz(), vec3(3.0, 2.0, 3.0));
    assert_eq!(v.zzx(), vec3(3.0, 3.0, 1.0));
    assert_eq!(v.zzy(), vec3(3.0, 3.0, 2.0));
    assert_eq!(v.zzz(), vec3(3.0, 3.0, 3.0));
    assert_eq!(v.xx(), vec2(1.0, 1.0));
    assert_eq!(v.xy(), vec2(1.0, 2.0));
    assert_eq!(v.xz(), vec2(1.0, 3.0));
    assert_eq!(v.yx(), vec2(2.0, 1.0));
    assert_eq!(v.yy(), vec2(2.0, 2.0));
    assert_eq!(v.yz(), vec2(2.0, 3.0));
    assert_eq!(v.zx(), vec2(3.0, 1.0));
    assert_eq!(v.zy(), vec2(3.0, 2.0));
    assert_eq!(v.zz(), vec2(3.0, 3.0));
}

#[test]
fn test_vec2_swizzles() {
    let v = vec2(1.0, 2.0);
    assert_eq!(v.xxxx(), vec4(1.0, 1.0, 1.0, 1.0));
    assert_eq!(v.xxxy(), vec4(1.0, 1.0, 1.0, 2.0));
    assert_eq!(v.xxyx(), vec4(1.0, 1.0, 2.0, 1.0));
    assert_eq!(v.xxyy(), vec4(1.0, 1.0, 2.0, 2.0));
    assert_eq!(v.xyxx(), vec4(1.0, 2.0, 1.0, 1.0));
    assert_eq!(v.xyxy(), vec4(1.0, 2.0, 1.0, 2.0));
    assert_eq!(v.xyyx(), vec4(1.0, 2.0, 2.0, 1.0));
    assert_eq!(v.xyyy(), vec4(1.0, 2.0, 2.0, 2.0));
    assert_eq!(v.yxxx(), vec4(2.0, 1.0, 1.0, 1.0));
    assert_eq!(v.yxxy(), vec4(2.0, 1.0, 1.0, 2.0));
    assert_eq!(v.yxyx(), vec4(2.0, 1.0, 2.0, 1.0));
    assert_eq!(v.yxyy(), vec4(2.0, 1.0, 2.0, 2.0));
    assert_eq!(v.yyxx(), vec4(2.0, 2.0, 1.0, 1.0));
    assert_eq!(v.yyxy(), vec4(2.0, 2.0, 1.0, 2.0));
    assert_eq!(v.yyyx(), vec4(2.0, 2.0, 2.0, 1.0));
    assert_eq!(v.yyyy(), vec4(2.0, 2.0, 2.0, 2.0));
    assert_eq!(v.xxx(), vec3(1.0, 1.0, 1.0));
    assert_eq!(v.xxy(), vec3(1.0, 1.0, 2.0));
    assert_eq!(v.xyx(), vec3(1.0, 2.0, 1.0));
    assert_eq!(v.xyy(), vec3(1.0, 2.0, 2.0));
    assert_eq!(v.yxx(), vec3(2.0, 1.0, 1.0));
    assert_eq!(v.yxy(), vec3(2.0, 1.0, 2.0));
    assert_eq!(v.yyx(), vec3(2.0, 2.0, 1.0));
    assert_eq!(v.yyy(), vec3(2.0, 2.0, 2.0));
    assert_eq!(v.xx(), vec2(1.0, 1.0));
    assert_eq!(v.yx(), vec2(2.0, 1.0));
    assert_eq!(v.yy(), vec2(2.0, 2.0));
}
