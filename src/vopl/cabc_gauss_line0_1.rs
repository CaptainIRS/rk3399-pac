#[doc = "Register `CABC_GAUSS_LINE0_1` reader"]
pub type R = crate::R<CabcGaussLine0_1Spec>;
#[doc = "Register `CABC_GAUSS_LINE0_1` writer"]
pub type W = crate::W<CabcGaussLine0_1Spec>;
#[doc = "Field `T_LINE0_4` reader - gauss parameter t_line0_4"]
pub type TLine0_4R = crate::FieldReader;
#[doc = "Field `T_LINE0_4` writer - gauss parameter t_line0_4"]
pub type TLine0_4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `T_LINE0_5` reader - gauss parameter t_line0_5"]
pub type TLine0_5R = crate::FieldReader;
#[doc = "Field `T_LINE0_5` writer - gauss parameter t_line0_5"]
pub type TLine0_5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `T_LINE0_6` reader - gauss parameter t_line0_6"]
pub type TLine0_6R = crate::FieldReader;
#[doc = "Field `T_LINE0_6` writer - gauss parameter t_line0_6"]
pub type TLine0_6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - gauss parameter t_line0_4"]
    #[inline(always)]
    pub fn t_line0_4(&self) -> TLine0_4R {
        TLine0_4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - gauss parameter t_line0_5"]
    #[inline(always)]
    pub fn t_line0_5(&self) -> TLine0_5R {
        TLine0_5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - gauss parameter t_line0_6"]
    #[inline(always)]
    pub fn t_line0_6(&self) -> TLine0_6R {
        TLine0_6R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - gauss parameter t_line0_4"]
    #[inline(always)]
    #[must_use]
    pub fn t_line0_4(&mut self) -> TLine0_4W<CabcGaussLine0_1Spec> {
        TLine0_4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - gauss parameter t_line0_5"]
    #[inline(always)]
    #[must_use]
    pub fn t_line0_5(&mut self) -> TLine0_5W<CabcGaussLine0_1Spec> {
        TLine0_5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - gauss parameter t_line0_6"]
    #[inline(always)]
    #[must_use]
    pub fn t_line0_6(&mut self) -> TLine0_6W<CabcGaussLine0_1Spec> {
        TLine0_6W::new(self, 16)
    }
}
#[doc = "CABC gauss line config register01\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cabc_gauss_line0_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cabc_gauss_line0_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CabcGaussLine0_1Spec;
impl crate::RegisterSpec for CabcGaussLine0_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cabc_gauss_line0_1::R`](R) reader structure"]
impl crate::Readable for CabcGaussLine0_1Spec {}
#[doc = "`write(|w| ..)` method takes [`cabc_gauss_line0_1::W`](W) writer structure"]
impl crate::Writable for CabcGaussLine0_1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CABC_GAUSS_LINE0_1 to value 0x0003_0911"]
impl crate::Resettable for CabcGaussLine0_1Spec {
    const RESET_VALUE: u32 = 0x0003_0911;
}
