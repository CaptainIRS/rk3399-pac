#[doc = "Register `ENH_C_COE` reader"]
pub type R = crate::R<EnhCCoeSpec>;
#[doc = "Register `ENH_C_COE` writer"]
pub type W = crate::W<EnhCCoeSpec>;
#[doc = "Field `C_FRAC_COE` reader - color enhancement fraction coefficient"]
pub type CFracCoeR = crate::FieldReader;
#[doc = "Field `C_FRAC_COE` writer - color enhancement fraction coefficient"]
pub type CFracCoeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `C_INT_COE` reader - color enhancement integer coefficient"]
pub type CIntCoeR = crate::FieldReader;
#[doc = "Field `C_INT_COE` writer - color enhancement integer coefficient"]
pub type CIntCoeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:4 - color enhancement fraction coefficient"]
    #[inline(always)]
    pub fn c_frac_coe(&self) -> CFracCoeR {
        CFracCoeR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - color enhancement integer coefficient"]
    #[inline(always)]
    pub fn c_int_coe(&self) -> CIntCoeR {
        CIntCoeR::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - color enhancement fraction coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn c_frac_coe(&mut self) -> CFracCoeW<EnhCCoeSpec> {
        CFracCoeW::new(self, 0)
    }
    #[doc = "Bits 5:6 - color enhancement integer coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn c_int_coe(&mut self) -> CIntCoeW<EnhCCoeSpec> {
        CIntCoeW::new(self, 5)
    }
}
#[doc = "rgb color enhancement coefficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enh_c_coe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enh_c_coe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnhCCoeSpec;
impl crate::RegisterSpec for EnhCCoeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enh_c_coe::R`](R) reader structure"]
impl crate::Readable for EnhCCoeSpec {}
#[doc = "`write(|w| ..)` method takes [`enh_c_coe::W`](W) writer structure"]
impl crate::Writable for EnhCCoeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENH_C_COE to value 0"]
impl crate::Resettable for EnhCCoeSpec {
    const RESET_VALUE: u32 = 0;
}
