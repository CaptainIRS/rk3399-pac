#[doc = "Register `GRF_USB20_PHY0_CON0` reader"]
pub type R = crate::R<GrfUsb20Phy0Con0Spec>;
#[doc = "Register `GRF_USB20_PHY0_CON0` writer"]
pub type W = crate::W<GrfUsb20Phy0Con0Spec>;
#[doc = "otg_disable_0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OtgDisable0 {
    #[doc = "1: disable otg function of usb3otg0"]
    B1 = 1,
    #[doc = "0: enable otg function of usb3otg0"]
    B0 = 0,
}
impl From<OtgDisable0> for bool {
    #[inline(always)]
    fn from(variant: OtgDisable0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTG_DISABLE_0` reader - otg_disable_0"]
pub type OtgDisable0R = crate::BitReader<OtgDisable0>;
impl OtgDisable0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OtgDisable0 {
        match self.bits {
            true => OtgDisable0::B1,
            false => OtgDisable0::B0,
        }
    }
    #[doc = "disable otg function of usb3otg0"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == OtgDisable0::B1
    }
    #[doc = "enable otg function of usb3otg0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == OtgDisable0::B0
    }
}
#[doc = "Field `OTG_DISABLE_0` writer - otg_disable_0"]
pub type OtgDisable0W<'a, REG> = crate::BitWriter<'a, REG, OtgDisable0>;
impl<'a, REG> OtgDisable0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable otg function of usb3otg0"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(OtgDisable0::B1)
    }
    #[doc = "enable otg function of usb3otg0"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(OtgDisable0::B0)
    }
}
#[doc = "otg_disable_1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OtgDisable1 {
    #[doc = "1: disable otg function of usb20 host0"]
    B1 = 1,
    #[doc = "0: enable otg function of usb20 host0"]
    B0 = 0,
}
impl From<OtgDisable1> for bool {
    #[inline(always)]
    fn from(variant: OtgDisable1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTG_DISABLE_1` reader - otg_disable_1"]
pub type OtgDisable1R = crate::BitReader<OtgDisable1>;
impl OtgDisable1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OtgDisable1 {
        match self.bits {
            true => OtgDisable1::B1,
            false => OtgDisable1::B0,
        }
    }
    #[doc = "disable otg function of usb20 host0"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == OtgDisable1::B1
    }
    #[doc = "enable otg function of usb20 host0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == OtgDisable1::B0
    }
}
#[doc = "Field `OTG_DISABLE_1` writer - otg_disable_1"]
pub type OtgDisable1W<'a, REG> = crate::BitWriter<'a, REG, OtgDisable1>;
impl<'a, REG> OtgDisable1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable otg function of usb20 host0"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(OtgDisable1::B1)
    }
    #[doc = "enable otg function of usb20 host0"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(OtgDisable1::B0)
    }
}
#[doc = "bypassdmen\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bypassdmen {
    #[doc = "1: enable bypass uart_sout to DM for usb3otg0"]
    B1 = 1,
    #[doc = "0: disable bypass uart_sout to DM for usb3otg0"]
    B0 = 0,
}
impl From<Bypassdmen> for bool {
    #[inline(always)]
    fn from(variant: Bypassdmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASSDMEN` reader - bypassdmen"]
pub type BypassdmenR = crate::BitReader<Bypassdmen>;
impl BypassdmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bypassdmen {
        match self.bits {
            true => Bypassdmen::B1,
            false => Bypassdmen::B0,
        }
    }
    #[doc = "enable bypass uart_sout to DM for usb3otg0"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bypassdmen::B1
    }
    #[doc = "disable bypass uart_sout to DM for usb3otg0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bypassdmen::B0
    }
}
#[doc = "Field `BYPASSDMEN` writer - bypassdmen"]
pub type BypassdmenW<'a, REG> = crate::BitWriter<'a, REG, Bypassdmen>;
impl<'a, REG> BypassdmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable bypass uart_sout to DM for usb3otg0"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bypassdmen::B1)
    }
    #[doc = "disable bypass uart_sout to DM for usb3otg0"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bypassdmen::B0)
    }
}
#[doc = "bypasssel\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bypasssel {
    #[doc = "1: bypass DP/DM as uart sin/sout for usb3otg0"]
    B1 = 1,
    #[doc = "0: Normal USB function for usb3otg0"]
    B0 = 0,
}
impl From<Bypasssel> for bool {
    #[inline(always)]
    fn from(variant: Bypasssel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASSSEL` reader - bypasssel"]
pub type BypassselR = crate::BitReader<Bypasssel>;
impl BypassselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bypasssel {
        match self.bits {
            true => Bypasssel::B1,
            false => Bypasssel::B0,
        }
    }
    #[doc = "bypass DP/DM as uart sin/sout for usb3otg0"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bypasssel::B1
    }
    #[doc = "Normal USB function for usb3otg0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bypasssel::B0
    }
}
#[doc = "Field `BYPASSSEL` writer - bypasssel"]
pub type BypassselW<'a, REG> = crate::BitWriter<'a, REG, Bypasssel>;
impl<'a, REG> BypassselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bypass DP/DM as uart sin/sout for usb3otg0"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bypasssel::B1)
    }
    #[doc = "Normal USB function for usb3otg0"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bypasssel::B0)
    }
}
#[doc = "Field `OTG_COMMONONN` reader - otg_commononn\n\nconfigure pll clock output in suspend mode"]
pub type OtgCommononnR = crate::BitReader;
#[doc = "Field `OTG_COMMONONN` writer - otg_commononn\n\nconfigure pll clock output in suspend mode"]
pub type OtgCommononnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDP_SINK_EN` reader - idp_sink_en\n\n1: enable idp_sink for battery charge for\n\nusb3otg0"]
pub type IdpSinkEnR = crate::BitReader;
#[doc = "Field `IDP_SINK_EN` writer - idp_sink_en\n\n1: enable idp_sink for battery charge for\n\nusb3otg0"]
pub type IdpSinkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDM_SINK_EN` reader - idm_sink_en\n\n1: enable idm_sink for battery charge for\n\nusb3otg0"]
pub type IdmSinkEnR = crate::BitReader;
#[doc = "Field `IDM_SINK_EN` writer - idm_sink_en\n\n1: enable idm_sink for battery charge for\n\nusb3otg0"]
pub type IdmSinkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDP_SRC_EN` reader - idp_src_en\n\n1: enable idp_src for battery charge for\n\nusb3otg0"]
pub type IdpSrcEnR = crate::BitReader;
#[doc = "Field `IDP_SRC_EN` writer - idp_src_en\n\n1: enable idp_src for battery charge for\n\nusb3otg0"]
pub type IdpSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDM_PDWN_EN` reader - rdm_pdwn_en\n\n1: enable rdm_pdwn for battery charge for\n\nusb3otg0"]
pub type RdmPdwnEnR = crate::BitReader;
#[doc = "Field `RDM_PDWN_EN` writer - rdm_pdwn_en\n\n1: enable rdm_pdwn for battery charge for\n\nusb3otg0"]
pub type RdmPdwnEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDP_SRC_EN` reader - vdp_src_en\n\n1:enable vdp_src for battery charge for\n\nusb3otg0"]
pub type VdpSrcEnR = crate::BitReader;
#[doc = "Field `VDP_SRC_EN` writer - vdp_src_en\n\n1:enable vdp_src for battery charge for\n\nusb3otg0"]
pub type VdpSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDM_SRC_EN` reader - vdm_src_en\n\n1: enable vdm_src for battery charge for\n\nusb3otg0"]
pub type VdmSrcEnR = crate::BitReader;
#[doc = "Field `VDM_SRC_EN` writer - vdm_src_en\n\n1: enable vdm_src for battery charge for\n\nusb3otg0"]
pub type VdmSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - otg_disable_0"]
    #[inline(always)]
    pub fn otg_disable_0(&self) -> OtgDisable0R {
        OtgDisable0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - otg_disable_1"]
    #[inline(always)]
    pub fn otg_disable_1(&self) -> OtgDisable1R {
        OtgDisable1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - bypassdmen"]
    #[inline(always)]
    pub fn bypassdmen(&self) -> BypassdmenR {
        BypassdmenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - bypasssel"]
    #[inline(always)]
    pub fn bypasssel(&self) -> BypassselR {
        BypassselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - otg_commononn\n\nconfigure pll clock output in suspend mode"]
    #[inline(always)]
    pub fn otg_commononn(&self) -> OtgCommononnR {
        OtgCommononnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - idp_sink_en\n\n1: enable idp_sink for battery charge for\n\nusb3otg0"]
    #[inline(always)]
    pub fn idp_sink_en(&self) -> IdpSinkEnR {
        IdpSinkEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - idm_sink_en\n\n1: enable idm_sink for battery charge for\n\nusb3otg0"]
    #[inline(always)]
    pub fn idm_sink_en(&self) -> IdmSinkEnR {
        IdmSinkEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - idp_src_en\n\n1: enable idp_src for battery charge for\n\nusb3otg0"]
    #[inline(always)]
    pub fn idp_src_en(&self) -> IdpSrcEnR {
        IdpSrcEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - rdm_pdwn_en\n\n1: enable rdm_pdwn for battery charge for\n\nusb3otg0"]
    #[inline(always)]
    pub fn rdm_pdwn_en(&self) -> RdmPdwnEnR {
        RdmPdwnEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - vdp_src_en\n\n1:enable vdp_src for battery charge for\n\nusb3otg0"]
    #[inline(always)]
    pub fn vdp_src_en(&self) -> VdpSrcEnR {
        VdpSrcEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - vdm_src_en\n\n1: enable vdm_src for battery charge for\n\nusb3otg0"]
    #[inline(always)]
    pub fn vdm_src_en(&self) -> VdmSrcEnR {
        VdmSrcEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - otg_disable_0"]
    #[inline(always)]
    #[must_use]
    pub fn otg_disable_0(&mut self) -> OtgDisable0W<GrfUsb20Phy0Con0Spec> {
        OtgDisable0W::new(self, 0)
    }
    #[doc = "Bit 1 - otg_disable_1"]
    #[inline(always)]
    #[must_use]
    pub fn otg_disable_1(&mut self) -> OtgDisable1W<GrfUsb20Phy0Con0Spec> {
        OtgDisable1W::new(self, 1)
    }
    #[doc = "Bit 2 - bypassdmen"]
    #[inline(always)]
    #[must_use]
    pub fn bypassdmen(&mut self) -> BypassdmenW<GrfUsb20Phy0Con0Spec> {
        BypassdmenW::new(self, 2)
    }
    #[doc = "Bit 3 - bypasssel"]
    #[inline(always)]
    #[must_use]
    pub fn bypasssel(&mut self) -> BypassselW<GrfUsb20Phy0Con0Spec> {
        BypassselW::new(self, 3)
    }
    #[doc = "Bit 4 - otg_commononn\n\nconfigure pll clock output in suspend mode"]
    #[inline(always)]
    #[must_use]
    pub fn otg_commononn(&mut self) -> OtgCommononnW<GrfUsb20Phy0Con0Spec> {
        OtgCommononnW::new(self, 4)
    }
    #[doc = "Bit 7 - idp_sink_en\n\n1: enable idp_sink for battery charge for\n\nusb3otg0"]
    #[inline(always)]
    #[must_use]
    pub fn idp_sink_en(&mut self) -> IdpSinkEnW<GrfUsb20Phy0Con0Spec> {
        IdpSinkEnW::new(self, 7)
    }
    #[doc = "Bit 8 - idm_sink_en\n\n1: enable idm_sink for battery charge for\n\nusb3otg0"]
    #[inline(always)]
    #[must_use]
    pub fn idm_sink_en(&mut self) -> IdmSinkEnW<GrfUsb20Phy0Con0Spec> {
        IdmSinkEnW::new(self, 8)
    }
    #[doc = "Bit 9 - idp_src_en\n\n1: enable idp_src for battery charge for\n\nusb3otg0"]
    #[inline(always)]
    #[must_use]
    pub fn idp_src_en(&mut self) -> IdpSrcEnW<GrfUsb20Phy0Con0Spec> {
        IdpSrcEnW::new(self, 9)
    }
    #[doc = "Bit 10 - rdm_pdwn_en\n\n1: enable rdm_pdwn for battery charge for\n\nusb3otg0"]
    #[inline(always)]
    #[must_use]
    pub fn rdm_pdwn_en(&mut self) -> RdmPdwnEnW<GrfUsb20Phy0Con0Spec> {
        RdmPdwnEnW::new(self, 10)
    }
    #[doc = "Bit 11 - vdp_src_en\n\n1:enable vdp_src for battery charge for\n\nusb3otg0"]
    #[inline(always)]
    #[must_use]
    pub fn vdp_src_en(&mut self) -> VdpSrcEnW<GrfUsb20Phy0Con0Spec> {
        VdpSrcEnW::new(self, 11)
    }
    #[doc = "Bit 12 - vdm_src_en\n\n1: enable vdm_src for battery charge for\n\nusb3otg0"]
    #[inline(always)]
    #[must_use]
    pub fn vdm_src_en(&mut self) -> VdmSrcEnW<GrfUsb20Phy0Con0Spec> {
        VdmSrcEnW::new(self, 12)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfUsb20Phy0Con0Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "USB20 PHY0 GRF Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb20_phy0_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb20_phy0_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfUsb20Phy0Con0Spec;
impl crate::RegisterSpec for GrfUsb20Phy0Con0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_usb20_phy0_con0::R`](R) reader structure"]
impl crate::Readable for GrfUsb20Phy0Con0Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_usb20_phy0_con0::W`](W) writer structure"]
impl crate::Writable for GrfUsb20Phy0Con0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_USB20_PHY0_CON0 to value 0"]
impl crate::Resettable for GrfUsb20Phy0Con0Spec {
    const RESET_VALUE: u32 = 0;
}
