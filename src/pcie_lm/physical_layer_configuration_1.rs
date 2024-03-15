#[doc = "Register `PHYSICAL_LAYER_CONFIGURATION_1` reader"]
pub type R = crate::R<PhysicalLayerConfiguration1Spec>;
#[doc = "Register `PHYSICAL_LAYER_CONFIGURATION_1` writer"]
pub type W = crate::W<PhysicalLayerConfiguration1Spec>;
#[doc = "Field `TLI` reader - Transmitted Link ID \\[TLI\\]
Link ID transmitted by the device in training sequences in the Root Port mode."]
pub type TliR = crate::FieldReader;
#[doc = "Field `TLI` writer - Transmitted Link ID \\[TLI\\]
Link ID transmitted by the device in training sequences in the Root Port mode."]
pub type TliW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TFC1` reader - Transmitted FTS Count at 2.5 GT/s Speed \\[TFC1\\]
FTS count transmitted by the core in TS1/TS2 sequences during link training. This value must be set based on the time needed by the receiver on the other side to acquire sync while exiting from L0S state."]
pub type Tfc1R = crate::FieldReader;
#[doc = "Field `TFC1` writer - Transmitted FTS Count at 2.5 GT/s Speed \\[TFC1\\]
FTS count transmitted by the core in TS1/TS2 sequences during link training. This value must be set based on the time needed by the receiver on the other side to acquire sync while exiting from L0S state."]
pub type Tfc1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TFC2` reader - Transmitted FTS Count at 5 GT/s Speed \\[TFC2\\]
FTS count transmitted by the core in TS1/TS2 sequences during link training. This value must be set based on the time needed by the receiver to acquire sync while exiting from L0S state."]
pub type Tfc2R = crate::FieldReader;
#[doc = "Field `TFC2` writer - Transmitted FTS Count at 5 GT/s Speed \\[TFC2\\]
FTS count transmitted by the core in TS1/TS2 sequences during link training. This value must be set based on the time needed by the receiver to acquire sync while exiting from L0S state."]
pub type Tfc2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TFC3` reader - Transmitted FTS Count at 8 GT/s Speed \\[TFC3\\]
FTS count transmitted by the core in TS1/TS2 sequences during link training. This value must be set based on the time needed by the receiver to acquire sync while exiting from L0S state."]
pub type Tfc3R = crate::FieldReader;
#[doc = "Field `TFC3` writer - Transmitted FTS Count at 8 GT/s Speed \\[TFC3\\]
FTS count transmitted by the core in TS1/TS2 sequences during link training. This value must be set based on the time needed by the receiver to acquire sync while exiting from L0S state."]
pub type Tfc3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmitted Link ID \\[TLI\\]
Link ID transmitted by the device in training sequences in the Root Port mode."]
    #[inline(always)]
    pub fn tli(&self) -> TliR {
        TliR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Transmitted FTS Count at 2.5 GT/s Speed \\[TFC1\\]
FTS count transmitted by the core in TS1/TS2 sequences during link training. This value must be set based on the time needed by the receiver on the other side to acquire sync while exiting from L0S state."]
    #[inline(always)]
    pub fn tfc1(&self) -> Tfc1R {
        Tfc1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Transmitted FTS Count at 5 GT/s Speed \\[TFC2\\]
FTS count transmitted by the core in TS1/TS2 sequences during link training. This value must be set based on the time needed by the receiver to acquire sync while exiting from L0S state."]
    #[inline(always)]
    pub fn tfc2(&self) -> Tfc2R {
        Tfc2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Transmitted FTS Count at 8 GT/s Speed \\[TFC3\\]
FTS count transmitted by the core in TS1/TS2 sequences during link training. This value must be set based on the time needed by the receiver to acquire sync while exiting from L0S state."]
    #[inline(always)]
    pub fn tfc3(&self) -> Tfc3R {
        Tfc3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmitted Link ID \\[TLI\\]
Link ID transmitted by the device in training sequences in the Root Port mode."]
    #[inline(always)]
    #[must_use]
    pub fn tli(&mut self) -> TliW<PhysicalLayerConfiguration1Spec> {
        TliW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Transmitted FTS Count at 2.5 GT/s Speed \\[TFC1\\]
FTS count transmitted by the core in TS1/TS2 sequences during link training. This value must be set based on the time needed by the receiver on the other side to acquire sync while exiting from L0S state."]
    #[inline(always)]
    #[must_use]
    pub fn tfc1(&mut self) -> Tfc1W<PhysicalLayerConfiguration1Spec> {
        Tfc1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Transmitted FTS Count at 5 GT/s Speed \\[TFC2\\]
FTS count transmitted by the core in TS1/TS2 sequences during link training. This value must be set based on the time needed by the receiver to acquire sync while exiting from L0S state."]
    #[inline(always)]
    #[must_use]
    pub fn tfc2(&mut self) -> Tfc2W<PhysicalLayerConfiguration1Spec> {
        Tfc2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Transmitted FTS Count at 8 GT/s Speed \\[TFC3\\]
FTS count transmitted by the core in TS1/TS2 sequences during link training. This value must be set based on the time needed by the receiver to acquire sync while exiting from L0S state."]
    #[inline(always)]
    #[must_use]
    pub fn tfc3(&mut self) -> Tfc3W<PhysicalLayerConfiguration1Spec> {
        Tfc3W::new(self, 24)
    }
}
#[doc = "Physical Layer Configuration Register 1 FTS count transmitted by the core in TS1/TS2 sequences during link training. This value must be set based on the time needed by the receiver to acquire sync while exiting from L0S state.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`physical_layer_configuration_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`physical_layer_configuration_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhysicalLayerConfiguration1Spec;
impl crate::RegisterSpec for PhysicalLayerConfiguration1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`physical_layer_configuration_1::R`](R) reader structure"]
impl crate::Readable for PhysicalLayerConfiguration1Spec {}
#[doc = "`write(|w| ..)` method takes [`physical_layer_configuration_1::W`](W) writer structure"]
impl crate::Writable for PhysicalLayerConfiguration1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PHYSICAL_LAYER_CONFIGURATION_1 to value 0x4080_8000"]
impl crate::Resettable for PhysicalLayerConfiguration1Spec {
    const RESET_VALUE: u32 = 0x4080_8000;
}
