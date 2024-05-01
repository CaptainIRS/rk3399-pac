#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "DCF start\n\nWrite 1 to start DCF. This bit will be cleared if DCF is stopped or DDR\n\nfrequency change is done.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    #[doc = "1: start DCF"]
    B1 = 1,
    #[doc = "0: no effect."]
    B0 = 0,
}
impl From<Start> for bool {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - DCF start\n\nWrite 1 to start DCF. This bit will be cleared if DCF is stopped or DDR\n\nfrequency change is done."]
pub type StartR = crate::BitReader<Start>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Start {
        match self.bits {
            true => Start::B1,
            false => Start::B0,
        }
    }
    #[doc = "start DCF"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Start::B1
    }
    #[doc = "no effect."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Start::B0
    }
}
#[doc = "Field `START` writer - DCF start\n\nWrite 1 to start DCF. This bit will be cleared if DCF is stopped or DDR\n\nfrequency change is done."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "start DCF"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Start::B1)
    }
    #[doc = "no effect."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Start::B0)
    }
}
#[doc = "DCF stop\n\nWrite 1 to stop DCF. Both DMA and AXI master will complete\n\ncurrent burst transfer and then stop. It may takes several cycles.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    #[doc = "1: DCF Stop ;"]
    B1 = 1,
    #[doc = "0: no effect ;"]
    B0 = 0,
}
impl From<Stop> for bool {
    #[inline(always)]
    fn from(variant: Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` reader - DCF stop\n\nWrite 1 to stop DCF. Both DMA and AXI master will complete\n\ncurrent burst transfer and then stop. It may takes several cycles."]
pub type StopR = crate::BitReader<Stop>;
impl StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stop {
        match self.bits {
            true => Stop::B1,
            false => Stop::B0,
        }
    }
    #[doc = "DCF Stop ;"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Stop::B1
    }
    #[doc = "no effect ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Stop::B0
    }
}
#[doc = "Field `STOP` writer - DCF stop\n\nWrite 1 to stop DCF. Both DMA and AXI master will complete\n\ncurrent burst transfer and then stop. It may takes several cycles."]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG, Stop>;
impl<'a, REG> StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DCF Stop ;"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::B1)
    }
    #[doc = "no effect ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::B0)
    }
}
#[doc = "DCF vop dma_finish ebable\n\nvop dma_finish hardware trigger enable signal\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VopHwEn {
    #[doc = "1: enable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<VopHwEn> for bool {
    #[inline(always)]
    fn from(variant: VopHwEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOP_HW_EN` reader - DCF vop dma_finish ebable\n\nvop dma_finish hardware trigger enable signal"]
pub type VopHwEnR = crate::BitReader<VopHwEn>;
impl VopHwEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VopHwEn {
        match self.bits {
            true => VopHwEn::B1,
            false => VopHwEn::B0,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VopHwEn::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VopHwEn::B0
    }
}
#[doc = "Field `VOP_HW_EN` writer - DCF vop dma_finish ebable\n\nvop dma_finish hardware trigger enable signal"]
pub type VopHwEnW<'a, REG> = crate::BitWriter<'a, REG, VopHwEn>;
impl<'a, REG> VopHwEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VopHwEn::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VopHwEn::B0)
    }
}
#[doc = "Field `BURST_LENGTH` reader - AXI burst length\n\nAXI transfer burst length\n\nfixed value 7 , do not modify"]
pub type BurstLengthR = crate::FieldReader;
#[doc = "Field `BURST_LENGTH` writer - AXI burst length\n\nAXI transfer burst length\n\nfixed value 7 , do not modify"]
pub type BurstLengthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - DCF start\n\nWrite 1 to start DCF. This bit will be cleared if DCF is stopped or DDR\n\nfrequency change is done."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCF stop\n\nWrite 1 to stop DCF. Both DMA and AXI master will complete\n\ncurrent burst transfer and then stop. It may takes several cycles."]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DCF vop dma_finish ebable\n\nvop dma_finish hardware trigger enable signal"]
    #[inline(always)]
    pub fn vop_hw_en(&self) -> VopHwEnR {
        VopHwEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - AXI burst length\n\nAXI transfer burst length\n\nfixed value 7 , do not modify"]
    #[inline(always)]
    pub fn burst_length(&self) -> BurstLengthR {
        BurstLengthR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DCF start\n\nWrite 1 to start DCF. This bit will be cleared if DCF is stopped or DDR\n\nfrequency change is done."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<CtrlSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - DCF stop\n\nWrite 1 to stop DCF. Both DMA and AXI master will complete\n\ncurrent burst transfer and then stop. It may takes several cycles."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<CtrlSpec> {
        StopW::new(self, 1)
    }
    #[doc = "Bit 2 - DCF vop dma_finish ebable\n\nvop dma_finish hardware trigger enable signal"]
    #[inline(always)]
    #[must_use]
    pub fn vop_hw_en(&mut self) -> VopHwEnW<CtrlSpec> {
        VopHwEnW::new(self, 2)
    }
    #[doc = "Bits 4:7 - AXI burst length\n\nAXI transfer burst length\n\nfixed value 7 , do not modify"]
    #[inline(always)]
    #[must_use]
    pub fn burst_length(&mut self) -> BurstLengthW<CtrlSpec> {
        BurstLengthW::new(self, 4)
    }
}
#[doc = "DCF Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x70"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x70;
}
