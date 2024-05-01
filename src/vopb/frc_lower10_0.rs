#[doc = "Register `FRC_LOWER10_0` reader"]
pub type R = crate::R<FrcLower10_0Spec>;
#[doc = "Register `FRC_LOWER10_0` writer"]
pub type W = crate::W<FrcLower10_0Spec>;
#[doc = "Field `LOWER10_FRM0` reader - frc parameter lowerbit = 2'b10,frm0"]
pub type Lower10Frm0R = crate::FieldReader<u16>;
#[doc = "Field `LOWER10_FRM0` writer - frc parameter lowerbit = 2'b10,frm0"]
pub type Lower10Frm0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LOWER10_FRM1` reader - frc parameter lowerbit = 2'b10,frm1"]
pub type Lower10Frm1R = crate::FieldReader<u16>;
#[doc = "Field `LOWER10_FRM1` writer - frc parameter lowerbit = 2'b10,frm1"]
pub type Lower10Frm1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - frc parameter lowerbit = 2'b10,frm0"]
    #[inline(always)]
    pub fn lower10_frm0(&self) -> Lower10Frm0R {
        Lower10Frm0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - frc parameter lowerbit = 2'b10,frm1"]
    #[inline(always)]
    pub fn lower10_frm1(&self) -> Lower10Frm1R {
        Lower10Frm1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - frc parameter lowerbit = 2'b10,frm0"]
    #[inline(always)]
    #[must_use]
    pub fn lower10_frm0(&mut self) -> Lower10Frm0W<FrcLower10_0Spec> {
        Lower10Frm0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - frc parameter lowerbit = 2'b10,frm1"]
    #[inline(always)]
    #[must_use]
    pub fn lower10_frm1(&mut self) -> Lower10Frm1W<FrcLower10_0Spec> {
        Lower10Frm1W::new(self, 16)
    }
}
#[doc = "FRC lookup table config register100\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frc_lower10_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frc_lower10_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrcLower10_0Spec;
impl crate::RegisterSpec for FrcLower10_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frc_lower10_0::R`](R) reader structure"]
impl crate::Readable for FrcLower10_0Spec {}
#[doc = "`write(|w| ..)` method takes [`frc_lower10_0::W`](W) writer structure"]
impl crate::Writable for FrcLower10_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRC_LOWER10_0 to value 0xa55a_9696"]
impl crate::Resettable for FrcLower10_0Spec {
    const RESET_VALUE: u32 = 0xa55a_9696;
}
