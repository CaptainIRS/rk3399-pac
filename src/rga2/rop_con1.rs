#[doc = "Register `ROP_CON1` reader"]
pub type R = crate::R<RopCon1Spec>;
#[doc = "Register `ROP_CON1` writer"]
pub type W = crate::W<RopCon1Spec>;
#[doc = "Field `SW_ROP3_CODE1` reader - Rop3 code 1 control bits"]
pub type SwRop3Code1R = crate::FieldReader<u32>;
#[doc = "Field `SW_ROP3_CODE1` writer - Rop3 code 1 control bits"]
pub type SwRop3Code1W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - Rop3 code 1 control bits"]
    #[inline(always)]
    pub fn sw_rop3_code1(&self) -> SwRop3Code1R {
        SwRop3Code1R::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - Rop3 code 1 control bits"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rop3_code1(&mut self) -> SwRop3Code1W<RopCon1Spec> {
        SwRop3Code1W::new(self, 0)
    }
}
#[doc = "ROP code 1 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rop_con1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rop_con1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RopCon1Spec;
impl crate::RegisterSpec for RopCon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rop_con1::R`](R) reader structure"]
impl crate::Readable for RopCon1Spec {}
#[doc = "`write(|w| ..)` method takes [`rop_con1::W`](W) writer structure"]
impl crate::Writable for RopCon1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROP_CON1 to value 0"]
impl crate::Resettable for RopCon1Spec {
    const RESET_VALUE: u32 = 0;
}
