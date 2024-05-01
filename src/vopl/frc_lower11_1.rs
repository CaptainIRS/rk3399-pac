#[doc = "Register `FRC_LOWER11_1` reader"]
pub type R = crate::R<FrcLower11_1Spec>;
#[doc = "Register `FRC_LOWER11_1` writer"]
pub type W = crate::W<FrcLower11_1Spec>;
#[doc = "Field `LOWER11_FRM2` reader - frc parameter lowerbit = 2'b11,frm2"]
pub type Lower11Frm2R = crate::FieldReader<u16>;
#[doc = "Field `LOWER11_FRM2` writer - frc parameter lowerbit = 2'b11,frm2"]
pub type Lower11Frm2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LOWER11_FRM3` reader - frc parameter lowerbit = 2'b11,frm3"]
pub type Lower11Frm3R = crate::FieldReader<u16>;
#[doc = "Field `LOWER11_FRM3` writer - frc parameter lowerbit = 2'b11,frm3"]
pub type Lower11Frm3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - frc parameter lowerbit = 2'b11,frm2"]
    #[inline(always)]
    pub fn lower11_frm2(&self) -> Lower11Frm2R {
        Lower11Frm2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - frc parameter lowerbit = 2'b11,frm3"]
    #[inline(always)]
    pub fn lower11_frm3(&self) -> Lower11Frm3R {
        Lower11Frm3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - frc parameter lowerbit = 2'b11,frm2"]
    #[inline(always)]
    #[must_use]
    pub fn lower11_frm2(&mut self) -> Lower11Frm2W<FrcLower11_1Spec> {
        Lower11Frm2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - frc parameter lowerbit = 2'b11,frm3"]
    #[inline(always)]
    #[must_use]
    pub fn lower11_frm3(&mut self) -> Lower11Frm3W<FrcLower11_1Spec> {
        Lower11Frm3W::new(self, 16)
    }
}
#[doc = "FRC lookup table config register111\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frc_lower11_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frc_lower11_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrcLower11_1Spec;
impl crate::RegisterSpec for FrcLower11_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frc_lower11_1::R`](R) reader structure"]
impl crate::Readable for FrcLower11_1Spec {}
#[doc = "`write(|w| ..)` method takes [`frc_lower11_1::W`](W) writer structure"]
impl crate::Writable for FrcLower11_1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRC_LOWER11_1 to value 0xed7b_b7de"]
impl crate::Resettable for FrcLower11_1Spec {
    const RESET_VALUE: u32 = 0xed7b_b7de;
}
