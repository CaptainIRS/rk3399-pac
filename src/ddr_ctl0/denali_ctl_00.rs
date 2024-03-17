#[doc = "Register `DENALI_CTL_00` reader"]
pub type R = crate::R<DenaliCtl00Spec>;
#[doc = "Register `DENALI_CTL_00` writer"]
pub type W = crate::W<DenaliCtl00Spec>;
#[doc = "Field `START` reader - Initiate command processing in the controller. Set to 1 to initiate."]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Initiate command processing in the controller. Set to 1 to initiate."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRAM_CLASS` reader - Defines the class of DRAM memory which is connected to the controller."]
pub type DramClassR = crate::FieldReader;
#[doc = "Field `DRAM_CLASS` writer - Defines the class of DRAM memory which is connected to the controller."]
pub type DramClassW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VERSION` reader - Holds the controller version number. READ-ONLY"]
pub type VersionR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Initiate command processing in the controller. Set to 1 to initiate."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - Defines the class of DRAM memory which is connected to the controller."]
    #[inline(always)]
    pub fn dram_class(&self) -> DramClassR {
        DramClassR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Holds the controller version number. READ-ONLY"]
    #[inline(always)]
    pub fn version(&self) -> VersionR {
        VersionR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Initiate command processing in the controller. Set to 1 to initiate."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<DenaliCtl00Spec> {
        StartW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Defines the class of DRAM memory which is connected to the controller."]
    #[inline(always)]
    #[must_use]
    pub fn dram_class(&mut self) -> DramClassW<DenaliCtl00Spec> {
        DramClassW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_00::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_00::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl00Spec;
impl crate::RegisterSpec for DenaliCtl00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_00::R`](R) reader structure"]
impl crate::Readable for DenaliCtl00Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_00::W`](W) writer structure"]
impl crate::Writable for DenaliCtl00Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_00 to value 0x2041_0000"]
impl crate::Resettable for DenaliCtl00Spec {
    const RESET_VALUE: u32 = 0x2041_0000;
}
