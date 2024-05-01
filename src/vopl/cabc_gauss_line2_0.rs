#[doc = "Register `CABC_GAUSS_LINE2_0` reader"]
pub type R = crate::R<CabcGaussLine2_0Spec>;
#[doc = "Register `CABC_GAUSS_LINE2_0` writer"]
pub type W = crate::W<CabcGaussLine2_0Spec>;
#[doc = "Field `T_LINE2_0` reader - gauss parameter t_line2_0"]
pub type TLine2_0R = crate::FieldReader;
#[doc = "Field `T_LINE2_0` writer - gauss parameter t_line2_0"]
pub type TLine2_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `T_LINE2_1` reader - gauss parameter t_line2_1"]
pub type TLine2_1R = crate::FieldReader;
#[doc = "Field `T_LINE2_1` writer - gauss parameter t_line2_1"]
pub type TLine2_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `T_LINE2_2` reader - gauss parameter t_line2_2"]
pub type TLine2_2R = crate::FieldReader;
#[doc = "Field `T_LINE2_2` writer - gauss parameter t_line2_2"]
pub type TLine2_2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `T_LINE2_3` reader - gauss parameter t_line2_3"]
pub type TLine2_3R = crate::FieldReader;
#[doc = "Field `T_LINE2_3` writer - gauss parameter t_line2_3"]
pub type TLine2_3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - gauss parameter t_line2_0"]
    #[inline(always)]
    pub fn t_line2_0(&self) -> TLine2_0R {
        TLine2_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - gauss parameter t_line2_1"]
    #[inline(always)]
    pub fn t_line2_1(&self) -> TLine2_1R {
        TLine2_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - gauss parameter t_line2_2"]
    #[inline(always)]
    pub fn t_line2_2(&self) -> TLine2_2R {
        TLine2_2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - gauss parameter t_line2_3"]
    #[inline(always)]
    pub fn t_line2_3(&self) -> TLine2_3R {
        TLine2_3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - gauss parameter t_line2_0"]
    #[inline(always)]
    #[must_use]
    pub fn t_line2_0(&mut self) -> TLine2_0W<CabcGaussLine2_0Spec> {
        TLine2_0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - gauss parameter t_line2_1"]
    #[inline(always)]
    #[must_use]
    pub fn t_line2_1(&mut self) -> TLine2_1W<CabcGaussLine2_0Spec> {
        TLine2_1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - gauss parameter t_line2_2"]
    #[inline(always)]
    #[must_use]
    pub fn t_line2_2(&mut self) -> TLine2_2W<CabcGaussLine2_0Spec> {
        TLine2_2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - gauss parameter t_line2_3"]
    #[inline(always)]
    #[must_use]
    pub fn t_line2_3(&mut self) -> TLine2_3W<CabcGaussLine2_0Spec> {
        TLine2_3W::new(self, 24)
    }
}
#[doc = "CABC gauss line config register20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cabc_gauss_line2_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cabc_gauss_line2_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CabcGaussLine2_0Spec;
impl crate::RegisterSpec for CabcGaussLine2_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cabc_gauss_line2_0::R`](R) reader structure"]
impl crate::Readable for CabcGaussLine2_0Spec {}
#[doc = "`write(|w| ..)` method takes [`cabc_gauss_line2_0::W`](W) writer structure"]
impl crate::Writable for CabcGaussLine2_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CABC_GAUSS_LINE2_0 to value 0x1511_0903"]
impl crate::Resettable for CabcGaussLine2_0Spec {
    const RESET_VALUE: u32 = 0x1511_0903;
}
