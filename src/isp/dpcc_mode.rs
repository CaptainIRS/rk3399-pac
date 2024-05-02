#[doc = "Register `DPCC_MODE` reader"]
pub type R = crate::R<DpccModeSpec>;
#[doc = "Register `DPCC_MODE` writer"]
pub type W = crate::W<DpccModeSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IspDpccEnable {
    #[doc = "1: enable DPCC"]
    B1 = 1,
    #[doc = "0: bypass DPCC *Default*"]
    B0 = 0,
}
impl From<IspDpccEnable> for bool {
    #[inline(always)]
    fn from(variant: IspDpccEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISP_DPCC_enable` reader - "]
pub type IspDpccEnableR = crate::BitReader<IspDpccEnable>;
impl IspDpccEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IspDpccEnable {
        match self.bits {
            true => IspDpccEnable::B1,
            false => IspDpccEnable::B0,
        }
    }
    #[doc = "enable DPCC"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IspDpccEnable::B1
    }
    #[doc = "bypass DPCC *Default*"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IspDpccEnable::B0
    }
}
#[doc = "Field `ISP_DPCC_enable` writer - "]
pub type IspDpccEnableW<'a, REG> = crate::BitWriter<'a, REG, IspDpccEnable>;
impl<'a, REG> IspDpccEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable DPCC"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IspDpccEnable::B1)
    }
    #[doc = "bypass DPCC *Default*"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IspDpccEnable::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrayscaleMode {
    #[doc = "1: enable gray scale data input from black and white sensors (without color filter array)"]
    B1 = 1,
    #[doc = "0: BAYER DATA INPUT *Default*"]
    B0 = 0,
}
impl From<GrayscaleMode> for bool {
    #[inline(always)]
    fn from(variant: GrayscaleMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GRAYSCALE_MODE` reader - "]
pub type GrayscaleModeR = crate::BitReader<GrayscaleMode>;
impl GrayscaleModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GrayscaleMode {
        match self.bits {
            true => GrayscaleMode::B1,
            false => GrayscaleMode::B0,
        }
    }
    #[doc = "enable gray scale data input from black and white sensors (without color filter array)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GrayscaleMode::B1
    }
    #[doc = "BAYER DATA INPUT *Default*"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GrayscaleMode::B0
    }
}
#[doc = "Field `GRAYSCALE_MODE` writer - "]
pub type GrayscaleModeW<'a, REG> = crate::BitWriter<'a, REG, GrayscaleMode>;
impl<'a, REG> GrayscaleModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable gray scale data input from black and white sensors (without color filter array)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GrayscaleMode::B1)
    }
    #[doc = "BAYER DATA INPUT *Default*"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GrayscaleMode::B0)
    }
}
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage1Enable {
    #[doc = "1: enable stage1 *Default*"]
    B1 = 1,
    #[doc = "0: bypass stage1"]
    B0 = 0,
}
impl From<Stage1Enable> for bool {
    #[inline(always)]
    fn from(variant: Stage1Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STAGE1_ENABLE` reader - "]
pub type Stage1EnableR = crate::BitReader<Stage1Enable>;
impl Stage1EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stage1Enable {
        match self.bits {
            true => Stage1Enable::B1,
            false => Stage1Enable::B0,
        }
    }
    #[doc = "enable stage1 *Default*"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Stage1Enable::B1
    }
    #[doc = "bypass stage1"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Stage1Enable::B0
    }
}
#[doc = "Field `STAGE1_ENABLE` writer - "]
pub type Stage1EnableW<'a, REG> = crate::BitWriter<'a, REG, Stage1Enable>;
impl<'a, REG> Stage1EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable stage1 *Default*"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Stage1Enable::B1)
    }
    #[doc = "bypass stage1"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Stage1Enable::B0)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn isp_dpcc_enable(&self) -> IspDpccEnableR {
        IspDpccEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn grayscale_mode(&self) -> GrayscaleModeR {
        GrayscaleModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn stage1_enable(&self) -> Stage1EnableR {
        Stage1EnableR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn isp_dpcc_enable(&mut self) -> IspDpccEnableW<DpccModeSpec> {
        IspDpccEnableW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn grayscale_mode(&mut self) -> GrayscaleModeW<DpccModeSpec> {
        GrayscaleModeW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn stage1_enable(&mut self) -> Stage1EnableW<DpccModeSpec> {
        Stage1EnableW::new(self, 2)
    }
}
#[doc = "Mode control for DPCC detection unit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpccModeSpec;
impl crate::RegisterSpec for DpccModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpcc_mode::R`](R) reader structure"]
impl crate::Readable for DpccModeSpec {}
#[doc = "`write(|w| ..)` method takes [`dpcc_mode::W`](W) writer structure"]
impl crate::Writable for DpccModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPCC_MODE to value 0x04"]
impl crate::Resettable for DpccModeSpec {
    const RESET_VALUE: u32 = 0x04;
}
