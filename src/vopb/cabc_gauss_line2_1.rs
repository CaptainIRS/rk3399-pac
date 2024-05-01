#[doc = "Register `CABC_GAUSS_LINE2_1` reader"]
pub type R = crate::R<CabcGaussLine2_1Spec>;
#[doc = "Register `CABC_GAUSS_LINE2_1` writer"]
pub type W = crate::W<CabcGaussLine2_1Spec>;
#[doc = "Field `T_LINE2_4` reader - gauss parameter t_line2_4"]
pub type TLine2_4R = crate::FieldReader;
#[doc = "Field `T_LINE2_4` writer - gauss parameter t_line2_4"]
pub type TLine2_4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `T_LINE2_5` reader - gauss parameter t_line2_5"]
pub type TLine2_5R = crate::FieldReader;
#[doc = "Field `T_LINE2_5` writer - gauss parameter t_line2_5"]
pub type TLine2_5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `T_LINE2_6` reader - gauss parameter t_line2_6"]
pub type TLine2_6R = crate::FieldReader;
#[doc = "Field `T_LINE2_6` writer - gauss parameter t_line2_6"]
pub type TLine2_6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - gauss parameter t_line2_4"]
    #[inline(always)]
    pub fn t_line2_4(&self) -> TLine2_4R {
        TLine2_4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - gauss parameter t_line2_5"]
    #[inline(always)]
    pub fn t_line2_5(&self) -> TLine2_5R {
        TLine2_5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - gauss parameter t_line2_6"]
    #[inline(always)]
    pub fn t_line2_6(&self) -> TLine2_6R {
        TLine2_6R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - gauss parameter t_line2_4"]
    #[inline(always)]
    #[must_use]
    pub fn t_line2_4(&mut self) -> TLine2_4W<CabcGaussLine2_1Spec> {
        TLine2_4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - gauss parameter t_line2_5"]
    #[inline(always)]
    #[must_use]
    pub fn t_line2_5(&mut self) -> TLine2_5W<CabcGaussLine2_1Spec> {
        TLine2_5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - gauss parameter t_line2_6"]
    #[inline(always)]
    #[must_use]
    pub fn t_line2_6(&mut self) -> TLine2_6W<CabcGaussLine2_1Spec> {
        TLine2_6W::new(self, 16)
    }
}
#[doc = "CABC gauss line config register21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cabc_gauss_line2_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cabc_gauss_line2_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CabcGaussLine2_1Spec;
impl crate::RegisterSpec for CabcGaussLine2_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cabc_gauss_line2_1::R`](R) reader structure"]
impl crate::Readable for CabcGaussLine2_1Spec {}
#[doc = "`write(|w| ..)` method takes [`cabc_gauss_line2_1::W`](W) writer structure"]
impl crate::Writable for CabcGaussLine2_1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CABC_GAUSS_LINE2_1 to value 0x0003_0911"]
impl crate::Resettable for CabcGaussLine2_1Spec {
    const RESET_VALUE: u32 = 0x0003_0911;
}
