#[doc = "Register `CABC_GAUSS_LINE1_0` reader"]
pub type R = crate::R<CabcGaussLine1_0Spec>;
#[doc = "Register `CABC_GAUSS_LINE1_0` writer"]
pub type W = crate::W<CabcGaussLine1_0Spec>;
#[doc = "Field `T_LINE1_0` reader - gauss parameter t_line1_0"]
pub type TLine1_0R = crate::FieldReader;
#[doc = "Field `T_LINE1_0` writer - gauss parameter t_line1_0"]
pub type TLine1_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `T_LINE1_1` reader - gauss parameter t_line1_1"]
pub type TLine1_1R = crate::FieldReader;
#[doc = "Field `T_LINE1_1` writer - gauss parameter t_line1_1"]
pub type TLine1_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `T_LINE1_2` reader - gauss parameter t_line1_2"]
pub type TLine1_2R = crate::FieldReader;
#[doc = "Field `T_LINE1_2` writer - gauss parameter t_line1_2"]
pub type TLine1_2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `T_LINE1_3` reader - gauss parameter t_line1_3"]
pub type TLine1_3R = crate::FieldReader;
#[doc = "Field `T_LINE1_3` writer - gauss parameter t_line1_3"]
pub type TLine1_3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - gauss parameter t_line1_0"]
    #[inline(always)]
    pub fn t_line1_0(&self) -> TLine1_0R {
        TLine1_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - gauss parameter t_line1_1"]
    #[inline(always)]
    pub fn t_line1_1(&self) -> TLine1_1R {
        TLine1_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - gauss parameter t_line1_2"]
    #[inline(always)]
    pub fn t_line1_2(&self) -> TLine1_2R {
        TLine1_2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - gauss parameter t_line1_3"]
    #[inline(always)]
    pub fn t_line1_3(&self) -> TLine1_3R {
        TLine1_3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - gauss parameter t_line1_0"]
    #[inline(always)]
    #[must_use]
    pub fn t_line1_0(&mut self) -> TLine1_0W<CabcGaussLine1_0Spec> {
        TLine1_0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - gauss parameter t_line1_1"]
    #[inline(always)]
    #[must_use]
    pub fn t_line1_1(&mut self) -> TLine1_1W<CabcGaussLine1_0Spec> {
        TLine1_1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - gauss parameter t_line1_2"]
    #[inline(always)]
    #[must_use]
    pub fn t_line1_2(&mut self) -> TLine1_2W<CabcGaussLine1_0Spec> {
        TLine1_2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - gauss parameter t_line1_3"]
    #[inline(always)]
    #[must_use]
    pub fn t_line1_3(&mut self) -> TLine1_3W<CabcGaussLine1_0Spec> {
        TLine1_3W::new(self, 24)
    }
}
#[doc = "CABC gauss line config register10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cabc_gauss_line1_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cabc_gauss_line1_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CabcGaussLine1_0Spec;
impl crate::RegisterSpec for CabcGaussLine1_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cabc_gauss_line1_0::R`](R) reader structure"]
impl crate::Readable for CabcGaussLine1_0Spec {}
#[doc = "`write(|w| ..)` method takes [`cabc_gauss_line1_0::W`](W) writer structure"]
impl crate::Writable for CabcGaussLine1_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CABC_GAUSS_LINE1_0 to value 0x1a15_0b04"]
impl crate::Resettable for CabcGaussLine1_0Spec {
    const RESET_VALUE: u32 = 0x1a15_0b04;
}
