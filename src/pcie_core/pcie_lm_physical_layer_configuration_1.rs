#[doc = "Register `PCIE_LM_PHYSICAL_LAYER_CONFIGURATION_1` reader"]
pub type R = crate::R<PcieLmPhysicalLayerConfiguration1Spec>;
#[doc = "Register `PCIE_LM_PHYSICAL_LAYER_CONFIGURATION_1` writer"]
pub type W = crate::W<PcieLmPhysicalLayerConfiguration1Spec>;
#[doc = "Field `TLI` reader - Transmitted Link ID \\[TLI\\]\n\nLink ID transmitted by the device in\n\ntraining sequences in the Root Port\n\nmode."]
pub type TliR = crate::FieldReader;
#[doc = "Field `TLI` writer - Transmitted Link ID \\[TLI\\]\n\nLink ID transmitted by the device in\n\ntraining sequences in the Root Port\n\nmode."]
pub type TliW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TFC1` reader - Transmitted FTS Count at 2.5 GT/s Speed \\[TFC1\\]\n\nFTS count transmitted by the core in\n\nTS1/TS2 sequences during link\n\ntraining. This value must be set\n\nbased on the time needed by the\n\nreceiver on the other side to acquire\n\nsync while exiting from L0S state."]
pub type Tfc1R = crate::FieldReader;
#[doc = "Field `TFC1` writer - Transmitted FTS Count at 2.5 GT/s Speed \\[TFC1\\]\n\nFTS count transmitted by the core in\n\nTS1/TS2 sequences during link\n\ntraining. This value must be set\n\nbased on the time needed by the\n\nreceiver on the other side to acquire\n\nsync while exiting from L0S state."]
pub type Tfc1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TFC2` reader - Transmitted FTS Count at 5 GT/s Speed \\[TFC2\\]\n\nFTS count transmitted by the core in\n\nTS1/TS2 sequences during link\n\ntraining. This value must be set\n\nbased on the time needed by the\n\nreceiver to acquire sync while exiting\n\nfrom L0S state."]
pub type Tfc2R = crate::FieldReader;
#[doc = "Field `TFC2` writer - Transmitted FTS Count at 5 GT/s Speed \\[TFC2\\]\n\nFTS count transmitted by the core in\n\nTS1/TS2 sequences during link\n\ntraining. This value must be set\n\nbased on the time needed by the\n\nreceiver to acquire sync while exiting\n\nfrom L0S state."]
pub type Tfc2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TFC3` reader - Transmitted FTS Count at 8 GT/s Speed \\[TFC3\\]\n\nFTS count transmitted by the core in\n\nTS1/TS2 sequences during link\n\ntraining. This value must be set\n\nbased on the time needed by the\n\nreceiver to acquire sync while exiting\n\nfrom L0S state."]
pub type Tfc3R = crate::FieldReader;
#[doc = "Field `TFC3` writer - Transmitted FTS Count at 8 GT/s Speed \\[TFC3\\]\n\nFTS count transmitted by the core in\n\nTS1/TS2 sequences during link\n\ntraining. This value must be set\n\nbased on the time needed by the\n\nreceiver to acquire sync while exiting\n\nfrom L0S state."]
pub type Tfc3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmitted Link ID \\[TLI\\]\n\nLink ID transmitted by the device in\n\ntraining sequences in the Root Port\n\nmode."]
    #[inline(always)]
    pub fn tli(&self) -> TliR {
        TliR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Transmitted FTS Count at 2.5 GT/s Speed \\[TFC1\\]\n\nFTS count transmitted by the core in\n\nTS1/TS2 sequences during link\n\ntraining. This value must be set\n\nbased on the time needed by the\n\nreceiver on the other side to acquire\n\nsync while exiting from L0S state."]
    #[inline(always)]
    pub fn tfc1(&self) -> Tfc1R {
        Tfc1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Transmitted FTS Count at 5 GT/s Speed \\[TFC2\\]\n\nFTS count transmitted by the core in\n\nTS1/TS2 sequences during link\n\ntraining. This value must be set\n\nbased on the time needed by the\n\nreceiver to acquire sync while exiting\n\nfrom L0S state."]
    #[inline(always)]
    pub fn tfc2(&self) -> Tfc2R {
        Tfc2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Transmitted FTS Count at 8 GT/s Speed \\[TFC3\\]\n\nFTS count transmitted by the core in\n\nTS1/TS2 sequences during link\n\ntraining. This value must be set\n\nbased on the time needed by the\n\nreceiver to acquire sync while exiting\n\nfrom L0S state."]
    #[inline(always)]
    pub fn tfc3(&self) -> Tfc3R {
        Tfc3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmitted Link ID \\[TLI\\]\n\nLink ID transmitted by the device in\n\ntraining sequences in the Root Port\n\nmode."]
    #[inline(always)]
    #[must_use]
    pub fn tli(&mut self) -> TliW<PcieLmPhysicalLayerConfiguration1Spec> {
        TliW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Transmitted FTS Count at 2.5 GT/s Speed \\[TFC1\\]\n\nFTS count transmitted by the core in\n\nTS1/TS2 sequences during link\n\ntraining. This value must be set\n\nbased on the time needed by the\n\nreceiver on the other side to acquire\n\nsync while exiting from L0S state."]
    #[inline(always)]
    #[must_use]
    pub fn tfc1(&mut self) -> Tfc1W<PcieLmPhysicalLayerConfiguration1Spec> {
        Tfc1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Transmitted FTS Count at 5 GT/s Speed \\[TFC2\\]\n\nFTS count transmitted by the core in\n\nTS1/TS2 sequences during link\n\ntraining. This value must be set\n\nbased on the time needed by the\n\nreceiver to acquire sync while exiting\n\nfrom L0S state."]
    #[inline(always)]
    #[must_use]
    pub fn tfc2(&mut self) -> Tfc2W<PcieLmPhysicalLayerConfiguration1Spec> {
        Tfc2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Transmitted FTS Count at 8 GT/s Speed \\[TFC3\\]\n\nFTS count transmitted by the core in\n\nTS1/TS2 sequences during link\n\ntraining. This value must be set\n\nbased on the time needed by the\n\nreceiver to acquire sync while exiting\n\nfrom L0S state."]
    #[inline(always)]
    #[must_use]
    pub fn tfc3(&mut self) -> Tfc3W<PcieLmPhysicalLayerConfiguration1Spec> {
        Tfc3W::new(self, 24)
    }
}
#[doc = "Physical Layer Configuration Register 1\n\nFTS count transmitted by the core in\n\nTS1/TS2 sequences during link\n\ntraining. This value must be set\n\nbased on the time needed by the\n\nreceiver to acquire sync while exiting\n\nfrom L0S state.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_physical_layer_configuration_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_physical_layer_configuration_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmPhysicalLayerConfiguration1Spec;
impl crate::RegisterSpec for PcieLmPhysicalLayerConfiguration1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_physical_layer_configuration_1::R`](R) reader structure"]
impl crate::Readable for PcieLmPhysicalLayerConfiguration1Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_physical_layer_configuration_1::W`](W) writer structure"]
impl crate::Writable for PcieLmPhysicalLayerConfiguration1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_LM_PHYSICAL_LAYER_CONFIGURATION_1 to value 0x4080_8000"]
impl crate::Resettable for PcieLmPhysicalLayerConfiguration1Spec {
    const RESET_VALUE: u32 = 0x4080_8000;
}
