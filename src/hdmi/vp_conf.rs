#[doc = "Register `VP_CONF` reader"]
pub type R = crate::R<VpConfSpec>;
#[doc = "Register `VP_CONF` writer"]
pub type W = crate::W<VpConfSpec>;
#[doc = "Field `OUTPUT_SELECTOR_0` reader - Video Packetizer output selection 0b: Data from\n\npixel packing block 1b: Data from YCC422 remap\n\nblock"]
pub type OutputSelector0R = crate::BitReader;
#[doc = "Field `OUTPUT_SELECTOR_0` writer - Video Packetizer output selection 0b: Data from\n\npixel packing block 1b: Data from YCC422 remap\n\nblock"]
pub type OutputSelector0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTPUT_SELECTOR` reader - When set to 1'b1, Data from pixel packing block."]
pub type OutputSelectorR = crate::BitReader;
#[doc = "Field `OUTPUT_SELECTOR` writer - When set to 1'b1, Data from pixel packing block."]
pub type OutputSelectorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "bypass_select\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BypassSelect {
    #[doc = "0: Data from pixel repeater block"]
    B0 = 0,
    #[doc = "1: Data from input of Video Packetizer block"]
    B1 = 1,
}
impl From<BypassSelect> for bool {
    #[inline(always)]
    fn from(variant: BypassSelect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASS_SELECT` reader - bypass_select"]
pub type BypassSelectR = crate::BitReader<BypassSelect>;
impl BypassSelectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BypassSelect {
        match self.bits {
            false => BypassSelect::B0,
            true => BypassSelect::B1,
        }
    }
    #[doc = "Data from pixel repeater block"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BypassSelect::B0
    }
    #[doc = "Data from input of Video Packetizer block"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BypassSelect::B1
    }
}
#[doc = "Field `BYPASS_SELECT` writer - bypass_select"]
pub type BypassSelectW<'a, REG> = crate::BitWriter<'a, REG, BypassSelect>;
impl<'a, REG> BypassSelectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data from pixel repeater block"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BypassSelect::B0)
    }
    #[doc = "Data from input of Video Packetizer block"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BypassSelect::B1)
    }
}
#[doc = "Field `YCC422_EN` reader - YCC 422 select enable. Disabling forces bypass\n\nmodule to output always zeros."]
pub type Ycc422EnR = crate::BitReader;
#[doc = "Field `YCC422_EN` writer - YCC 422 select enable. Disabling forces bypass\n\nmodule to output always zeros."]
pub type Ycc422EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR_EN` reader - Pixel repeater enable. When set to 0, the pixel\n\nrepetition block is disabled."]
pub type PrEnR = crate::BitReader;
#[doc = "Field `PR_EN` writer - Pixel repeater enable. When set to 0, the pixel\n\nrepetition block is disabled."]
pub type PrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PP_EN` reader - Pixel packing enable. When set to 0, the pixel\n\npacking block is disabled if bypass_en is 1’b0."]
pub type PpEnR = crate::BitReader;
#[doc = "Field `PP_EN` writer - Pixel packing enable. When set to 0, the pixel\n\npacking block is disabled if bypass_en is 1’b0."]
pub type PpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPASS_EN` reader - When set to 1'b1, Pixel packing enable. When set\n\nto 1b'0, the pixel packing block is controlled by\n\npp_en."]
pub type BypassEnR = crate::BitReader;
#[doc = "Field `BYPASS_EN` writer - When set to 1'b1, Pixel packing enable. When set\n\nto 1b'0, the pixel packing block is controlled by\n\npp_en."]
pub type BypassEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Video Packetizer output selection 0b: Data from\n\npixel packing block 1b: Data from YCC422 remap\n\nblock"]
    #[inline(always)]
    pub fn output_selector_0(&self) -> OutputSelector0R {
        OutputSelector0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set to 1'b1, Data from pixel packing block."]
    #[inline(always)]
    pub fn output_selector(&self) -> OutputSelectorR {
        OutputSelectorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - bypass_select"]
    #[inline(always)]
    pub fn bypass_select(&self) -> BypassSelectR {
        BypassSelectR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - YCC 422 select enable. Disabling forces bypass\n\nmodule to output always zeros."]
    #[inline(always)]
    pub fn ycc422_en(&self) -> Ycc422EnR {
        Ycc422EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pixel repeater enable. When set to 0, the pixel\n\nrepetition block is disabled."]
    #[inline(always)]
    pub fn pr_en(&self) -> PrEnR {
        PrEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pixel packing enable. When set to 0, the pixel\n\npacking block is disabled if bypass_en is 1’b0."]
    #[inline(always)]
    pub fn pp_en(&self) -> PpEnR {
        PpEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When set to 1'b1, Pixel packing enable. When set\n\nto 1b'0, the pixel packing block is controlled by\n\npp_en."]
    #[inline(always)]
    pub fn bypass_en(&self) -> BypassEnR {
        BypassEnR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Video Packetizer output selection 0b: Data from\n\npixel packing block 1b: Data from YCC422 remap\n\nblock"]
    #[inline(always)]
    #[must_use]
    pub fn output_selector_0(&mut self) -> OutputSelector0W<VpConfSpec> {
        OutputSelector0W::new(self, 0)
    }
    #[doc = "Bit 1 - When set to 1'b1, Data from pixel packing block."]
    #[inline(always)]
    #[must_use]
    pub fn output_selector(&mut self) -> OutputSelectorW<VpConfSpec> {
        OutputSelectorW::new(self, 1)
    }
    #[doc = "Bit 2 - bypass_select"]
    #[inline(always)]
    #[must_use]
    pub fn bypass_select(&mut self) -> BypassSelectW<VpConfSpec> {
        BypassSelectW::new(self, 2)
    }
    #[doc = "Bit 3 - YCC 422 select enable. Disabling forces bypass\n\nmodule to output always zeros."]
    #[inline(always)]
    #[must_use]
    pub fn ycc422_en(&mut self) -> Ycc422EnW<VpConfSpec> {
        Ycc422EnW::new(self, 3)
    }
    #[doc = "Bit 4 - Pixel repeater enable. When set to 0, the pixel\n\nrepetition block is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn pr_en(&mut self) -> PrEnW<VpConfSpec> {
        PrEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Pixel packing enable. When set to 0, the pixel\n\npacking block is disabled if bypass_en is 1’b0."]
    #[inline(always)]
    #[must_use]
    pub fn pp_en(&mut self) -> PpEnW<VpConfSpec> {
        PpEnW::new(self, 5)
    }
    #[doc = "Bit 6 - When set to 1'b1, Pixel packing enable. When set\n\nto 1b'0, the pixel packing block is controlled by\n\npp_en."]
    #[inline(always)]
    #[must_use]
    pub fn bypass_en(&mut self) -> BypassEnW<VpConfSpec> {
        BypassEnW::new(self, 6)
    }
}
#[doc = "Video Packetizer Output and Enable Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vp_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vp_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VpConfSpec;
impl crate::RegisterSpec for VpConfSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vp_conf::R`](R) reader structure"]
impl crate::Readable for VpConfSpec {}
#[doc = "`write(|w| ..)` method takes [`vp_conf::W`](W) writer structure"]
impl crate::Writable for VpConfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets VP_CONF to value 0x24"]
impl crate::Resettable for VpConfSpec {
    const RESET_VALUE: u8 = 0x24;
}
