#[doc = "Register `DENALI_CTL_163` reader"]
pub type R = crate::R<DenaliCtl163Spec>;
#[doc = "Register `DENALI_CTL_163` writer"]
pub type W = crate::W<DenaliCtl163Spec>;
#[doc = "Field `FSP0_FRC_VALID` reader - Specifies whether the FSP set defined in the FSP0_FRC parameter reflects the frequency used to program the FSP0 registers."]
pub type Fsp0FrcValidR = crate::BitReader;
#[doc = "Field `FSP0_FRC_VALID` writer - Specifies whether the FSP set defined in the FSP0_FRC parameter reflects the frequency used to program the FSP0 registers."]
pub type Fsp0FrcValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSP1_FRC_VALID` reader - Specifies whether the FSP set defined in the FSP1_FRC parameter reflects the frequency used to program the FSP1 registers."]
pub type Fsp1FrcValidR = crate::BitReader;
#[doc = "Field `FSP1_FRC_VALID` writer - Specifies whether the FSP set defined in the FSP1_FRC parameter reflects the frequency used to program the FSP1 registers."]
pub type Fsp1FrcValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSP0_FRC` reader - Identifies which of the controllersfrequencycopyisassociat edwithFSP0.'"]
pub type Fsp0FrcR = crate::FieldReader;
#[doc = "Field `FSP0_FRC` writer - Identifies which of the controllersfrequencycopyisassociat edwithFSP0.'"]
pub type Fsp0FrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FSP1_FRC` reader - Identifies which of the controllersfrequencycopyisassociat edwithFSP1.'"]
pub type Fsp1FrcR = crate::FieldReader;
#[doc = "Field `FSP1_FRC` writer - Identifies which of the controllersfrequencycopyisassociat edwithFSP1.'"]
pub type Fsp1FrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Specifies whether the FSP set defined in the FSP0_FRC parameter reflects the frequency used to program the FSP0 registers."]
    #[inline(always)]
    pub fn fsp0_frc_valid(&self) -> Fsp0FrcValidR {
        Fsp0FrcValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Specifies whether the FSP set defined in the FSP1_FRC parameter reflects the frequency used to program the FSP1 registers."]
    #[inline(always)]
    pub fn fsp1_frc_valid(&self) -> Fsp1FrcValidR {
        Fsp1FrcValidR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Identifies which of the controllersfrequencycopyisassociat edwithFSP0.'"]
    #[inline(always)]
    pub fn fsp0_frc(&self) -> Fsp0FrcR {
        Fsp0FrcR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Identifies which of the controllersfrequencycopyisassociat edwithFSP1.'"]
    #[inline(always)]
    pub fn fsp1_frc(&self) -> Fsp1FrcR {
        Fsp1FrcR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies whether the FSP set defined in the FSP0_FRC parameter reflects the frequency used to program the FSP0 registers."]
    #[inline(always)]
    #[must_use]
    pub fn fsp0_frc_valid(&mut self) -> Fsp0FrcValidW<DenaliCtl163Spec> {
        Fsp0FrcValidW::new(self, 0)
    }
    #[doc = "Bit 8 - Specifies whether the FSP set defined in the FSP1_FRC parameter reflects the frequency used to program the FSP1 registers."]
    #[inline(always)]
    #[must_use]
    pub fn fsp1_frc_valid(&mut self) -> Fsp1FrcValidW<DenaliCtl163Spec> {
        Fsp1FrcValidW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Identifies which of the controllersfrequencycopyisassociat edwithFSP0.'"]
    #[inline(always)]
    #[must_use]
    pub fn fsp0_frc(&mut self) -> Fsp0FrcW<DenaliCtl163Spec> {
        Fsp0FrcW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Identifies which of the controllersfrequencycopyisassociat edwithFSP1.'"]
    #[inline(always)]
    #[must_use]
    pub fn fsp1_frc(&mut self) -> Fsp1FrcW<DenaliCtl163Spec> {
        Fsp1FrcW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_163::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_163::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl163Spec;
impl crate::RegisterSpec for DenaliCtl163Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_163::R`](R) reader structure"]
impl crate::Readable for DenaliCtl163Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_163::W`](W) writer structure"]
impl crate::Writable for DenaliCtl163Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_163 to value 0"]
impl crate::Resettable for DenaliCtl163Spec {
    const RESET_VALUE: u32 = 0;
}
