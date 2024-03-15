#[doc = "Register `VP_PR_CD` reader"]
pub type R = crate::R<VpPrCdSpec>;
#[doc = "Register `VP_PR_CD` writer"]
pub type W = crate::W<VpPrCdSpec>;
#[doc = "Desired pixel repetition factor configuration. The configured value sets H13T PHY PLL to multiply pixel clock by the factor in order to obtain the desired repetition clock. For the CEA modes some are already defined with pixel repetition in the input video. So for CEA modes this shall be always 0. Shall only be used if the user wants to do pixel repetition using H13TCTRL controller. The action is stated corresponding to desired_pr_factor\\[3:0\\]:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DesiredPrFactor {
    #[doc = "0: Pixel sent 10 times Other: Reserved. Not used"]
    B0000 = 0,
    #[doc = "1: Pixel sent 10 times Other: Reserved. Not used"]
    B0001 = 1,
    #[doc = "2: Pixel sent 10 times Other: Reserved. Not used"]
    B0010 = 2,
    #[doc = "3: Pixel sent 10 times Other: Reserved. Not used"]
    B0011 = 3,
    #[doc = "4: Pixel sent 10 times Other: Reserved. Not used"]
    B0100 = 4,
    #[doc = "5: Pixel sent 10 times Other: Reserved. Not used"]
    B0101 = 5,
    #[doc = "6: Pixel sent 10 times Other: Reserved. Not used"]
    B0110 = 6,
    #[doc = "7: Pixel sent 10 times Other: Reserved. Not used"]
    B0111 = 7,
    #[doc = "8: Pixel sent 10 times Other: Reserved. Not used"]
    B1000 = 8,
    #[doc = "9: Pixel sent 10 times Other: Reserved. Not used"]
    B1001 = 9,
}
impl From<DesiredPrFactor> for u8 {
    #[inline(always)]
    fn from(variant: DesiredPrFactor) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DesiredPrFactor {
    type Ux = u8;
}
#[doc = "Field `DESIRED_PR_FACTOR` reader - Desired pixel repetition factor configuration. The configured value sets H13T PHY PLL to multiply pixel clock by the factor in order to obtain the desired repetition clock. For the CEA modes some are already defined with pixel repetition in the input video. So for CEA modes this shall be always 0. Shall only be used if the user wants to do pixel repetition using H13TCTRL controller. The action is stated corresponding to desired_pr_factor\\[3:0\\]:"]
pub type DesiredPrFactorR = crate::FieldReader<DesiredPrFactor>;
impl DesiredPrFactorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DesiredPrFactor> {
        match self.bits {
            0 => Some(DesiredPrFactor::B0000),
            1 => Some(DesiredPrFactor::B0001),
            2 => Some(DesiredPrFactor::B0010),
            3 => Some(DesiredPrFactor::B0011),
            4 => Some(DesiredPrFactor::B0100),
            5 => Some(DesiredPrFactor::B0101),
            6 => Some(DesiredPrFactor::B0110),
            7 => Some(DesiredPrFactor::B0111),
            8 => Some(DesiredPrFactor::B1000),
            9 => Some(DesiredPrFactor::B1001),
            _ => None,
        }
    }
    #[doc = "Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == DesiredPrFactor::B0000
    }
    #[doc = "Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == DesiredPrFactor::B0001
    }
    #[doc = "Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn is_b0010(&self) -> bool {
        *self == DesiredPrFactor::B0010
    }
    #[doc = "Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn is_b0011(&self) -> bool {
        *self == DesiredPrFactor::B0011
    }
    #[doc = "Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn is_b0100(&self) -> bool {
        *self == DesiredPrFactor::B0100
    }
    #[doc = "Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn is_b0101(&self) -> bool {
        *self == DesiredPrFactor::B0101
    }
    #[doc = "Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn is_b0110(&self) -> bool {
        *self == DesiredPrFactor::B0110
    }
    #[doc = "Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn is_b0111(&self) -> bool {
        *self == DesiredPrFactor::B0111
    }
    #[doc = "Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn is_b1000(&self) -> bool {
        *self == DesiredPrFactor::B1000
    }
    #[doc = "Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn is_b1001(&self) -> bool {
        *self == DesiredPrFactor::B1001
    }
}
#[doc = "Field `DESIRED_PR_FACTOR` writer - Desired pixel repetition factor configuration. The configured value sets H13T PHY PLL to multiply pixel clock by the factor in order to obtain the desired repetition clock. For the CEA modes some are already defined with pixel repetition in the input video. So for CEA modes this shall be always 0. Shall only be used if the user wants to do pixel repetition using H13TCTRL controller. The action is stated corresponding to desired_pr_factor\\[3:0\\]:"]
pub type DesiredPrFactorW<'a, REG> = crate::FieldWriter<'a, REG, 4, DesiredPrFactor>;
impl<'a, REG> DesiredPrFactorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(DesiredPrFactor::B0000)
    }
    #[doc = "Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn b0001(self) -> &'a mut crate::W<REG> {
        self.variant(DesiredPrFactor::B0001)
    }
    #[doc = "Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn b0010(self) -> &'a mut crate::W<REG> {
        self.variant(DesiredPrFactor::B0010)
    }
    #[doc = "Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn b0011(self) -> &'a mut crate::W<REG> {
        self.variant(DesiredPrFactor::B0011)
    }
    #[doc = "Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn b0100(self) -> &'a mut crate::W<REG> {
        self.variant(DesiredPrFactor::B0100)
    }
    #[doc = "Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn b0101(self) -> &'a mut crate::W<REG> {
        self.variant(DesiredPrFactor::B0101)
    }
    #[doc = "Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn b0110(self) -> &'a mut crate::W<REG> {
        self.variant(DesiredPrFactor::B0110)
    }
    #[doc = "Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn b0111(self) -> &'a mut crate::W<REG> {
        self.variant(DesiredPrFactor::B0111)
    }
    #[doc = "Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn b1000(self) -> &'a mut crate::W<REG> {
        self.variant(DesiredPrFactor::B1000)
    }
    #[doc = "Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn b1001(self) -> &'a mut crate::W<REG> {
        self.variant(DesiredPrFactor::B1001)
    }
}
#[doc = "The Color depth configuration is described as the following, with the action stated corresponding to color_depth\\[3:0\\]:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ColorDepth {
    #[doc = "0: 48 bits per pixel video (16 bits per component). 16-bit packing mode. Other: Reserved. Not used."]
    B0000 = 0,
    #[doc = "4: 48 bits per pixel video (16 bits per component). 16-bit packing mode. Other: Reserved. Not used."]
    B0100 = 4,
    #[doc = "5: 48 bits per pixel video (16 bits per component). 16-bit packing mode. Other: Reserved. Not used."]
    B0101 = 5,
    #[doc = "6: 48 bits per pixel video (16 bits per component). 16-bit packing mode. Other: Reserved. Not used."]
    B0110 = 6,
    #[doc = "7: 48 bits per pixel video (16 bits per component). 16-bit packing mode. Other: Reserved. Not used."]
    B0111 = 7,
}
impl From<ColorDepth> for u8 {
    #[inline(always)]
    fn from(variant: ColorDepth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ColorDepth {
    type Ux = u8;
}
#[doc = "Field `COLOR_DEPTH` reader - The Color depth configuration is described as the following, with the action stated corresponding to color_depth\\[3:0\\]:"]
pub type ColorDepthR = crate::FieldReader<ColorDepth>;
impl ColorDepthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ColorDepth> {
        match self.bits {
            0 => Some(ColorDepth::B0000),
            4 => Some(ColorDepth::B0100),
            5 => Some(ColorDepth::B0101),
            6 => Some(ColorDepth::B0110),
            7 => Some(ColorDepth::B0111),
            _ => None,
        }
    }
    #[doc = "48 bits per pixel video (16 bits per component). 16-bit packing mode. Other: Reserved. Not used."]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == ColorDepth::B0000
    }
    #[doc = "48 bits per pixel video (16 bits per component). 16-bit packing mode. Other: Reserved. Not used."]
    #[inline(always)]
    pub fn is_b0100(&self) -> bool {
        *self == ColorDepth::B0100
    }
    #[doc = "48 bits per pixel video (16 bits per component). 16-bit packing mode. Other: Reserved. Not used."]
    #[inline(always)]
    pub fn is_b0101(&self) -> bool {
        *self == ColorDepth::B0101
    }
    #[doc = "48 bits per pixel video (16 bits per component). 16-bit packing mode. Other: Reserved. Not used."]
    #[inline(always)]
    pub fn is_b0110(&self) -> bool {
        *self == ColorDepth::B0110
    }
    #[doc = "48 bits per pixel video (16 bits per component). 16-bit packing mode. Other: Reserved. Not used."]
    #[inline(always)]
    pub fn is_b0111(&self) -> bool {
        *self == ColorDepth::B0111
    }
}
#[doc = "Field `COLOR_DEPTH` writer - The Color depth configuration is described as the following, with the action stated corresponding to color_depth\\[3:0\\]:"]
pub type ColorDepthW<'a, REG> = crate::FieldWriter<'a, REG, 4, ColorDepth>;
impl<'a, REG> ColorDepthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "48 bits per pixel video (16 bits per component). 16-bit packing mode. Other: Reserved. Not used."]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(ColorDepth::B0000)
    }
    #[doc = "48 bits per pixel video (16 bits per component). 16-bit packing mode. Other: Reserved. Not used."]
    #[inline(always)]
    pub fn b0100(self) -> &'a mut crate::W<REG> {
        self.variant(ColorDepth::B0100)
    }
    #[doc = "48 bits per pixel video (16 bits per component). 16-bit packing mode. Other: Reserved. Not used."]
    #[inline(always)]
    pub fn b0101(self) -> &'a mut crate::W<REG> {
        self.variant(ColorDepth::B0101)
    }
    #[doc = "48 bits per pixel video (16 bits per component). 16-bit packing mode. Other: Reserved. Not used."]
    #[inline(always)]
    pub fn b0110(self) -> &'a mut crate::W<REG> {
        self.variant(ColorDepth::B0110)
    }
    #[doc = "48 bits per pixel video (16 bits per component). 16-bit packing mode. Other: Reserved. Not used."]
    #[inline(always)]
    pub fn b0111(self) -> &'a mut crate::W<REG> {
        self.variant(ColorDepth::B0111)
    }
}
impl R {
    #[doc = "Bits 0:3 - Desired pixel repetition factor configuration. The configured value sets H13T PHY PLL to multiply pixel clock by the factor in order to obtain the desired repetition clock. For the CEA modes some are already defined with pixel repetition in the input video. So for CEA modes this shall be always 0. Shall only be used if the user wants to do pixel repetition using H13TCTRL controller. The action is stated corresponding to desired_pr_factor\\[3:0\\]:"]
    #[inline(always)]
    pub fn desired_pr_factor(&self) -> DesiredPrFactorR {
        DesiredPrFactorR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - The Color depth configuration is described as the following, with the action stated corresponding to color_depth\\[3:0\\]:"]
    #[inline(always)]
    pub fn color_depth(&self) -> ColorDepthR {
        ColorDepthR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Desired pixel repetition factor configuration. The configured value sets H13T PHY PLL to multiply pixel clock by the factor in order to obtain the desired repetition clock. For the CEA modes some are already defined with pixel repetition in the input video. So for CEA modes this shall be always 0. Shall only be used if the user wants to do pixel repetition using H13TCTRL controller. The action is stated corresponding to desired_pr_factor\\[3:0\\]:"]
    #[inline(always)]
    #[must_use]
    pub fn desired_pr_factor(&mut self) -> DesiredPrFactorW<VpPrCdSpec> {
        DesiredPrFactorW::new(self, 0)
    }
    #[doc = "Bits 4:7 - The Color depth configuration is described as the following, with the action stated corresponding to color_depth\\[3:0\\]:"]
    #[inline(always)]
    #[must_use]
    pub fn color_depth(&mut self) -> ColorDepthW<VpPrCdSpec> {
        ColorDepthW::new(self, 4)
    }
}
#[doc = "Desired pixel repetition factor configuration. The configured value sets H13T PHY PLL to multiply pixel clock by the factor in order to obtain the desired repetition clock. For the CEA modes some are already defined with pixel repetition in the input video. So for CEA modes this shall be always 0. Shall only be used if the user wants to do pixel repetition using H13TCTRL controller. The action is stated corresponding to desired_pr_factor\\[3:0\\]: 0000b: No pixel repetition (pixel sent only once) 0001b: Pixel sent two times (pixel repeated once) 0010b: Pixel sent three times 0011b: Pixel sent four times 0100b: Pixel sent five times 0101b: Pixel sent six times 0110b: Pixel sent seven times 0111b: Pixel sent eight times 1000b: Pixel sent nine times 1001b: Pixel sent 10 times Other: Reserved. Not used\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vp_pr_cd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vp_pr_cd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VpPrCdSpec;
impl crate::RegisterSpec for VpPrCdSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vp_pr_cd::R`](R) reader structure"]
impl crate::Readable for VpPrCdSpec {}
#[doc = "`write(|w| ..)` method takes [`vp_pr_cd::W`](W) writer structure"]
impl crate::Writable for VpPrCdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets VP_PR_CD to value 0"]
impl crate::Resettable for VpPrCdSpec {
    const RESET_VALUE: u8 = 0;
}
