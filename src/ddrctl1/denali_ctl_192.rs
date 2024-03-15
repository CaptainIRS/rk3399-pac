#[doc = "Register `DENALI_CTL_192` reader"]
pub type R = crate::R<DenaliCtl192Spec>;
#[doc = "Register `DENALI_CTL_192` writer"]
pub type W = crate::W<DenaliCtl192Spec>;
#[doc = "Field `AGE_COUNT` reader - Initial value of master aging-rate counter for command aging."]
pub type AgeCountR = crate::FieldReader;
#[doc = "Field `AGE_COUNT` writer - Initial value of master aging-rate counter for command aging."]
pub type AgeCountW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `COMMAND_AGE_COUNT` reader - Initial value of individual command aging counters for command aging."]
pub type CommandAgeCountR = crate::FieldReader;
#[doc = "Field `COMMAND_AGE_COUNT` writer - Initial value of individual command aging counters for command aging."]
pub type CommandAgeCountW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDR_CMP_EN` reader - Enable address collision detection as a rule for command queue placement. Set to 1 to enable."]
pub type AddrCmpEnR = crate::BitReader;
#[doc = "Field `ADDR_CMP_EN` writer - Enable address collision detection as a rule for command queue placement. Set to 1 to enable."]
pub type AddrCmpEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Initial value of master aging-rate counter for command aging."]
    #[inline(always)]
    pub fn age_count(&self) -> AgeCountR {
        AgeCountR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Initial value of individual command aging counters for command aging."]
    #[inline(always)]
    pub fn command_age_count(&self) -> CommandAgeCountR {
        CommandAgeCountR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Enable address collision detection as a rule for command queue placement. Set to 1 to enable."]
    #[inline(always)]
    pub fn addr_cmp_en(&self) -> AddrCmpEnR {
        AddrCmpEnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Initial value of master aging-rate counter for command aging."]
    #[inline(always)]
    #[must_use]
    pub fn age_count(&mut self) -> AgeCountW<DenaliCtl192Spec> {
        AgeCountW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Initial value of individual command aging counters for command aging."]
    #[inline(always)]
    #[must_use]
    pub fn command_age_count(&mut self) -> CommandAgeCountW<DenaliCtl192Spec> {
        CommandAgeCountW::new(self, 8)
    }
    #[doc = "Bit 16 - Enable address collision detection as a rule for command queue placement. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn addr_cmp_en(&mut self) -> AddrCmpEnW<DenaliCtl192Spec> {
        AddrCmpEnW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_192::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_192::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl192Spec;
impl crate::RegisterSpec for DenaliCtl192Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_192::R`](R) reader structure"]
impl crate::Readable for DenaliCtl192Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_192::W`](W) writer structure"]
impl crate::Writable for DenaliCtl192Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_192 to value 0"]
impl crate::Resettable for DenaliCtl192Spec {
    const RESET_VALUE: u32 = 0;
}
