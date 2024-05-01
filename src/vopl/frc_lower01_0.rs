#[doc = "Register `FRC_LOWER01_0` reader"]
pub type R = crate::R<FrcLower01_0Spec>;
#[doc = "Register `FRC_LOWER01_0` writer"]
pub type W = crate::W<FrcLower01_0Spec>;
#[doc = "Field `LOWER01_FRM0` reader - frc parameter lowerbit = 2'b01,frm0"]
pub type Lower01Frm0R = crate::FieldReader<u16>;
#[doc = "Field `LOWER01_FRM0` writer - frc parameter lowerbit = 2'b01,frm0"]
pub type Lower01Frm0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LOWER01_FRM1` reader - frc parameter lowerbit = 2'b01,frm1"]
pub type Lower01Frm1R = crate::FieldReader<u16>;
#[doc = "Field `LOWER01_FRM1` writer - frc parameter lowerbit = 2'b01,frm1"]
pub type Lower01Frm1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - frc parameter lowerbit = 2'b01,frm0"]
    #[inline(always)]
    pub fn lower01_frm0(&self) -> Lower01Frm0R {
        Lower01Frm0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - frc parameter lowerbit = 2'b01,frm1"]
    #[inline(always)]
    pub fn lower01_frm1(&self) -> Lower01Frm1R {
        Lower01Frm1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - frc parameter lowerbit = 2'b01,frm0"]
    #[inline(always)]
    #[must_use]
    pub fn lower01_frm0(&mut self) -> Lower01Frm0W<FrcLower01_0Spec> {
        Lower01Frm0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - frc parameter lowerbit = 2'b01,frm1"]
    #[inline(always)]
    #[must_use]
    pub fn lower01_frm1(&mut self) -> Lower01Frm1W<FrcLower01_0Spec> {
        Lower01Frm1W::new(self, 16)
    }
}
#[doc = "FRC lookup table config register010\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frc_lower01_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frc_lower01_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrcLower01_0Spec;
impl crate::RegisterSpec for FrcLower01_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frc_lower01_0::R`](R) reader structure"]
impl crate::Readable for FrcLower01_0Spec {}
#[doc = "`write(|w| ..)` method takes [`frc_lower01_0::W`](W) writer structure"]
impl crate::Writable for FrcLower01_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRC_LOWER01_0 to value 0x1284_4821"]
impl crate::Resettable for FrcLower01_0Spec {
    const RESET_VALUE: u32 = 0x1284_4821;
}
