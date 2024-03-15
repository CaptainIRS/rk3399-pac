#[doc = "Register `ROOT_CONTROL_AND_CAPABILITY` reader"]
pub type R = crate::R<RootControlAndCapabilitySpec>;
#[doc = "Register `ROOT_CONTROL_AND_CAPABILITY` writer"]
pub type W = crate::W<RootControlAndCapabilitySpec>;
#[doc = "Field `SECEE` reader - System Error on Correctable Error Enable \\[SECEE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
pub type SeceeR = crate::BitReader;
#[doc = "Field `SECEE` writer - System Error on Correctable Error Enable \\[SECEE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
pub type SeceeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENFEE` reader - System Error on Non-Fatal Error Enable \\[SENFEE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
pub type SenfeeR = crate::BitReader;
#[doc = "Field `SENFEE` writer - System Error on Non-Fatal Error Enable \\[SENFEE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
pub type SenfeeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMEIE` reader - PME Interrupt Enable \\[PMEIE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
pub type PmeieR = crate::BitReader;
#[doc = "Field `PMEIE` writer - PME Interrupt Enable \\[PMEIE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
pub type PmeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRSSVE` reader - CRS Software Visibility Enable \\[CRSSVE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
pub type CrssveR = crate::BitReader;
#[doc = "Field `CRSSVE` writer - CRS Software Visibility Enable \\[CRSSVE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
pub type CrssveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R28` reader - Reserved \\[R28\\]
Reserved"]
pub type R28R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - System Error on Correctable Error Enable \\[SECEE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
    #[inline(always)]
    pub fn secee(&self) -> SeceeR {
        SeceeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - System Error on Non-Fatal Error Enable \\[SENFEE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
    #[inline(always)]
    pub fn senfee(&self) -> SenfeeR {
        SenfeeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PME Interrupt Enable \\[PMEIE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
    #[inline(always)]
    pub fn pmeie(&self) -> PmeieR {
        PmeieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CRS Software Visibility Enable \\[CRSSVE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
    #[inline(always)]
    pub fn crssve(&self) -> CrssveR {
        CrssveR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - Reserved \\[R28\\]
Reserved"]
    #[inline(always)]
    pub fn r28(&self) -> R28R {
        R28R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - System Error on Correctable Error Enable \\[SECEE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
    #[inline(always)]
    #[must_use]
    pub fn secee(&mut self) -> SeceeW<RootControlAndCapabilitySpec> {
        SeceeW::new(self, 0)
    }
    #[doc = "Bit 1 - System Error on Non-Fatal Error Enable \\[SENFEE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
    #[inline(always)]
    #[must_use]
    pub fn senfee(&mut self) -> SenfeeW<RootControlAndCapabilitySpec> {
        SenfeeW::new(self, 1)
    }
    #[doc = "Bit 2 - PME Interrupt Enable \\[PMEIE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
    #[inline(always)]
    #[must_use]
    pub fn pmeie(&mut self) -> PmeieW<RootControlAndCapabilitySpec> {
        PmeieW::new(self, 2)
    }
    #[doc = "Bit 3 - CRS Software Visibility Enable \\[CRSSVE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
    #[inline(always)]
    #[must_use]
    pub fn crssve(&mut self) -> CrssveW<RootControlAndCapabilitySpec> {
        CrssveW::new(self, 3)
    }
}
#[doc = "Root Control and Capability Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`root_control_and_capability::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`root_control_and_capability::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RootControlAndCapabilitySpec;
impl crate::RegisterSpec for RootControlAndCapabilitySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`root_control_and_capability::R`](R) reader structure"]
impl crate::Readable for RootControlAndCapabilitySpec {}
#[doc = "`write(|w| ..)` method takes [`root_control_and_capability::W`](W) writer structure"]
impl crate::Writable for RootControlAndCapabilitySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROOT_CONTROL_AND_CAPABILITY to value 0"]
impl crate::Resettable for RootControlAndCapabilitySpec {
    const RESET_VALUE: u32 = 0;
}
