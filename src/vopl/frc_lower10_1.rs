#[doc = "Register `FRC_LOWER10_1` reader"]
pub type R = crate::R<FrcLower10_1Spec>;
#[doc = "Register `FRC_LOWER10_1` writer"]
pub type W = crate::W<FrcLower10_1Spec>;
#[doc = "Field `LOWER10_FRM2` reader - frc parameter lowerbit = 2'b10,frm2"]
pub type Lower10Frm2R = crate::FieldReader<u16>;
#[doc = "Field `LOWER10_FRM2` writer - frc parameter lowerbit = 2'b10,frm2"]
pub type Lower10Frm2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LOWER10_FRM3` reader - frc parameter lowerbit = 2'b10,frm3"]
pub type Lower10Frm3R = crate::FieldReader<u16>;
#[doc = "Field `LOWER10_FRM3` writer - frc parameter lowerbit = 2'b10,frm3"]
pub type Lower10Frm3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - frc parameter lowerbit = 2'b10,frm2"]
    #[inline(always)]
    pub fn lower10_frm2(&self) -> Lower10Frm2R {
        Lower10Frm2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - frc parameter lowerbit = 2'b10,frm3"]
    #[inline(always)]
    pub fn lower10_frm3(&self) -> Lower10Frm3R {
        Lower10Frm3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - frc parameter lowerbit = 2'b10,frm2"]
    #[inline(always)]
    #[must_use]
    pub fn lower10_frm2(&mut self) -> Lower10Frm2W<FrcLower10_1Spec> {
        Lower10Frm2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - frc parameter lowerbit = 2'b10,frm3"]
    #[inline(always)]
    #[must_use]
    pub fn lower10_frm3(&mut self) -> Lower10Frm3W<FrcLower10_1Spec> {
        Lower10Frm3W::new(self, 16)
    }
}
#[doc = "FRC lookup table config register101\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frc_lower10_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frc_lower10_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrcLower10_1Spec;
impl crate::RegisterSpec for FrcLower10_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frc_lower10_1::R`](R) reader structure"]
impl crate::Readable for FrcLower10_1Spec {}
#[doc = "`write(|w| ..)` method takes [`frc_lower10_1::W`](W) writer structure"]
impl crate::Writable for FrcLower10_1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRC_LOWER10_1 to value 0x5aa5_6969"]
impl crate::Resettable for FrcLower10_1Spec {
    const RESET_VALUE: u32 = 0x5aa5_6969;
}
