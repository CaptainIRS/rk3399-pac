#[doc = "Register `GRF_SOC_STATUS3` reader"]
pub type R = crate::R<GrfSocStatus3Spec>;
#[doc = "Register `GRF_SOC_STATUS3` writer"]
pub type W = crate::W<GrfSocStatus3Spec>;
#[doc = "Field `USB20_PHY0_STAT_DP_ATTACHED` reader - usb20_phy0_stat_dp_attached status bit"]
pub type Usb20Phy0StatDpAttachedR = crate::BitReader;
#[doc = "Field `USB20_PHY0_STAT_DP_ATTACHED` writer - usb20_phy0_stat_dp_attached status bit"]
pub type Usb20Phy0StatDpAttachedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB20_PHY0_STAT_DCP_DETECTED` reader - usb20_phy0_stat_dcp_detected status bit"]
pub type Usb20Phy0StatDcpDetectedR = crate::BitReader;
#[doc = "Field `USB20_PHY0_STAT_DCP_DETECTED` writer - usb20_phy0_stat_dcp_detected status bit"]
pub type Usb20Phy0StatDcpDetectedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB20_PHY0_STAT_CP_DETECTED` reader - usb20_phy0_stat_cp_detected status bit"]
pub type Usb20Phy0StatCpDetectedR = crate::BitReader;
#[doc = "Field `USB20_PHY0_STAT_CP_DETECTED` writer - usb20_phy0_stat_cp_detected status bit"]
pub type Usb20Phy0StatCpDetectedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB20_PHY1_STAT_DP_ATTACHED` reader - usb20_phy1_stat_dp_attached status bit"]
pub type Usb20Phy1StatDpAttachedR = crate::BitReader;
#[doc = "Field `USB20_PHY1_STAT_DP_ATTACHED` writer - usb20_phy1_stat_dp_attached status bit"]
pub type Usb20Phy1StatDpAttachedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB20_PHY1_STAT_DCP_DETECTED` reader - usb20_phy1_stat_dcp_detected status bit"]
pub type Usb20Phy1StatDcpDetectedR = crate::BitReader;
#[doc = "Field `USB20_PHY1_STAT_DCP_DETECTED` writer - usb20_phy1_stat_dcp_detected status bit"]
pub type Usb20Phy1StatDcpDetectedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB20_PHY1_STAT_CP_DETECTED` reader - usb20_phy1_stat_cp_detected status bit"]
pub type Usb20Phy1StatCpDetectedR = crate::BitReader;
#[doc = "Field `USB20_PHY1_STAT_CP_DETECTED` writer - usb20_phy1_stat_cp_detected status bit"]
pub type Usb20Phy1StatCpDetectedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBCPHY0_OTG_UTMI_SESSEND` reader - usbcphy0_otg_utmi_sessend status bit"]
pub type Usbcphy0OtgUtmiSessendR = crate::BitReader;
#[doc = "Field `USBCPHY0_OTG_UTMI_SESSEND` writer - usbcphy0_otg_utmi_sessend status bit"]
pub type Usbcphy0OtgUtmiSessendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBCPHY0_OTG_UTMI_AVALID` reader - usbcphy0_otg_utmi_avalid status bit"]
pub type Usbcphy0OtgUtmiAvalidR = crate::BitReader;
#[doc = "Field `USBCPHY0_OTG_UTMI_AVALID` writer - usbcphy0_otg_utmi_avalid status bit"]
pub type Usbcphy0OtgUtmiAvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBCPHY0_OTG_UTMI_IDDIG` reader - usbcphy0_otg_utmi_iddig status bit"]
pub type Usbcphy0OtgUtmiIddigR = crate::BitReader;
#[doc = "Field `USBCPHY0_OTG_UTMI_IDDIG` writer - usbcphy0_otg_utmi_iddig status bit"]
pub type Usbcphy0OtgUtmiIddigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBCPHY1_OTG_UTMI_SESSEND` reader - usbcphy1_otg_utmi_sessend status bit"]
pub type Usbcphy1OtgUtmiSessendR = crate::BitReader;
#[doc = "Field `USBCPHY1_OTG_UTMI_SESSEND` writer - usbcphy1_otg_utmi_sessend status bit"]
pub type Usbcphy1OtgUtmiSessendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBCPHY1_OTG_UTMI_AVALID` reader - usbcphy1_otg_utmi_avalid status bit"]
pub type Usbcphy1OtgUtmiAvalidR = crate::BitReader;
#[doc = "Field `USBCPHY1_OTG_UTMI_AVALID` writer - usbcphy1_otg_utmi_avalid status bit"]
pub type Usbcphy1OtgUtmiAvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBCPHY1_OTG_UTMI_IDDIG` reader - usbcphy1_otg_utmi_iddig status bit"]
pub type Usbcphy1OtgUtmiIddigR = crate::BitReader;
#[doc = "Field `USBCPHY1_OTG_UTMI_IDDIG` writer - usbcphy1_otg_utmi_iddig status bit"]
pub type Usbcphy1OtgUtmiIddigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBCPHY0_OTG_UTMI_BVALID` reader - usbcphy0_otg_utmi_bvalid status bit"]
pub type Usbcphy0OtgUtmiBvalidR = crate::BitReader;
#[doc = "Field `USBCPHY0_OTG_UTMI_LINESTATE` reader - usbcphy0_otg_utmi_linestate status bit"]
pub type Usbcphy0OtgUtmiLinestateR = crate::FieldReader;
#[doc = "Field `USBCPHY0_OTG_UTMI_HOSTDISCONNECT` reader - usbcphy0_otg_utmi_hostdisconnect status"]
pub type Usbcphy0OtgUtmiHostdisconnectR = crate::BitReader;
#[doc = "Field `USBCPHY1_OTG_UTMI_BVALID` reader - usbcphy1_otg_utmi_bvalid status bit"]
pub type Usbcphy1OtgUtmiBvalidR = crate::BitReader;
#[doc = "Field `USBCPHY1_OTG_UTMI_LINESTATE` reader - usbcphy1_otg_utmi_linestate status bit"]
pub type Usbcphy1OtgUtmiLinestateR = crate::FieldReader;
#[doc = "Field `USBCPHY1_OTG_UTMI_HOSTDISCONNECT` reader - usbcphy1_otg_utmi_hostdisconnect status"]
pub type Usbcphy1OtgUtmiHostdisconnectR = crate::BitReader;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbcphy0HostUtmiFsXverOwn {
    #[doc = "1: ehci owns usb2phy"]
    B1 = 1,
    #[doc = "0: ehci owns usb2phy"]
    B0 = 0,
}
impl From<Usbcphy0HostUtmiFsXverOwn> for bool {
    #[inline(always)]
    fn from(variant: Usbcphy0HostUtmiFsXverOwn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBCPHY0_HOST_UTMI_FS_XVER_OWN` reader - "]
pub type Usbcphy0HostUtmiFsXverOwnR = crate::BitReader<Usbcphy0HostUtmiFsXverOwn>;
impl Usbcphy0HostUtmiFsXverOwnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbcphy0HostUtmiFsXverOwn {
        match self.bits {
            true => Usbcphy0HostUtmiFsXverOwn::B1,
            false => Usbcphy0HostUtmiFsXverOwn::B0,
        }
    }
    #[doc = "ehci owns usb2phy"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Usbcphy0HostUtmiFsXverOwn::B1
    }
    #[doc = "ehci owns usb2phy"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Usbcphy0HostUtmiFsXverOwn::B0
    }
}
#[doc = "Field `USBCPHY0_HOST_UTMI_LINESTATE` reader - usbcphy0_host_utmi_linestate status"]
pub type Usbcphy0HostUtmiLinestateR = crate::FieldReader;
#[doc = "Field `USBCPHY0_HOST_UTMI_HOSTDISCONNECT` reader - usbcphy0_host_utmi_hostdisconnect status"]
pub type Usbcphy0HostUtmiHostdisconnectR = crate::BitReader;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbcphy1HostUtmiFsXverOwn {
    #[doc = "1: ehci owns usb2phy"]
    B1 = 1,
    #[doc = "0: ehci owns usb2phy"]
    B0 = 0,
}
impl From<Usbcphy1HostUtmiFsXverOwn> for bool {
    #[inline(always)]
    fn from(variant: Usbcphy1HostUtmiFsXverOwn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBCPHY1_HOST_UTMI_FS_XVER_OWN` reader - "]
pub type Usbcphy1HostUtmiFsXverOwnR = crate::BitReader<Usbcphy1HostUtmiFsXverOwn>;
impl Usbcphy1HostUtmiFsXverOwnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbcphy1HostUtmiFsXverOwn {
        match self.bits {
            true => Usbcphy1HostUtmiFsXverOwn::B1,
            false => Usbcphy1HostUtmiFsXverOwn::B0,
        }
    }
    #[doc = "ehci owns usb2phy"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Usbcphy1HostUtmiFsXverOwn::B1
    }
    #[doc = "ehci owns usb2phy"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Usbcphy1HostUtmiFsXverOwn::B0
    }
}
#[doc = "Field `USBCPHY1_HOST_UTMI_LINESTATE` reader - usbcphy1_host_utmi_linestate status"]
pub type Usbcphy1HostUtmiLinestateR = crate::FieldReader;
#[doc = "Field `USBCPHY1_HOST_UTMI_HOSTDISCONNECT` reader - usbcphy1_host_utmi_hostdisconnect status"]
pub type Usbcphy1HostUtmiHostdisconnectR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - usb20_phy0_stat_dp_attached status bit"]
    #[inline(always)]
    pub fn usb20_phy0_stat_dp_attached(&self) -> Usb20Phy0StatDpAttachedR {
        Usb20Phy0StatDpAttachedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - usb20_phy0_stat_dcp_detected status bit"]
    #[inline(always)]
    pub fn usb20_phy0_stat_dcp_detected(&self) -> Usb20Phy0StatDcpDetectedR {
        Usb20Phy0StatDcpDetectedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - usb20_phy0_stat_cp_detected status bit"]
    #[inline(always)]
    pub fn usb20_phy0_stat_cp_detected(&self) -> Usb20Phy0StatCpDetectedR {
        Usb20Phy0StatCpDetectedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - usb20_phy1_stat_dp_attached status bit"]
    #[inline(always)]
    pub fn usb20_phy1_stat_dp_attached(&self) -> Usb20Phy1StatDpAttachedR {
        Usb20Phy1StatDpAttachedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - usb20_phy1_stat_dcp_detected status bit"]
    #[inline(always)]
    pub fn usb20_phy1_stat_dcp_detected(&self) -> Usb20Phy1StatDcpDetectedR {
        Usb20Phy1StatDcpDetectedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - usb20_phy1_stat_cp_detected status bit"]
    #[inline(always)]
    pub fn usb20_phy1_stat_cp_detected(&self) -> Usb20Phy1StatCpDetectedR {
        Usb20Phy1StatCpDetectedR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - usbcphy0_otg_utmi_sessend status bit"]
    #[inline(always)]
    pub fn usbcphy0_otg_utmi_sessend(&self) -> Usbcphy0OtgUtmiSessendR {
        Usbcphy0OtgUtmiSessendR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - usbcphy0_otg_utmi_avalid status bit"]
    #[inline(always)]
    pub fn usbcphy0_otg_utmi_avalid(&self) -> Usbcphy0OtgUtmiAvalidR {
        Usbcphy0OtgUtmiAvalidR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - usbcphy0_otg_utmi_iddig status bit"]
    #[inline(always)]
    pub fn usbcphy0_otg_utmi_iddig(&self) -> Usbcphy0OtgUtmiIddigR {
        Usbcphy0OtgUtmiIddigR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - usbcphy1_otg_utmi_sessend status bit"]
    #[inline(always)]
    pub fn usbcphy1_otg_utmi_sessend(&self) -> Usbcphy1OtgUtmiSessendR {
        Usbcphy1OtgUtmiSessendR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - usbcphy1_otg_utmi_avalid status bit"]
    #[inline(always)]
    pub fn usbcphy1_otg_utmi_avalid(&self) -> Usbcphy1OtgUtmiAvalidR {
        Usbcphy1OtgUtmiAvalidR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - usbcphy1_otg_utmi_iddig status bit"]
    #[inline(always)]
    pub fn usbcphy1_otg_utmi_iddig(&self) -> Usbcphy1OtgUtmiIddigR {
        Usbcphy1OtgUtmiIddigR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - usbcphy0_otg_utmi_bvalid status bit"]
    #[inline(always)]
    pub fn usbcphy0_otg_utmi_bvalid(&self) -> Usbcphy0OtgUtmiBvalidR {
        Usbcphy0OtgUtmiBvalidR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - usbcphy0_otg_utmi_linestate status bit"]
    #[inline(always)]
    pub fn usbcphy0_otg_utmi_linestate(&self) -> Usbcphy0OtgUtmiLinestateR {
        Usbcphy0OtgUtmiLinestateR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - usbcphy0_otg_utmi_hostdisconnect status"]
    #[inline(always)]
    pub fn usbcphy0_otg_utmi_hostdisconnect(&self) -> Usbcphy0OtgUtmiHostdisconnectR {
        Usbcphy0OtgUtmiHostdisconnectR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - usbcphy1_otg_utmi_bvalid status bit"]
    #[inline(always)]
    pub fn usbcphy1_otg_utmi_bvalid(&self) -> Usbcphy1OtgUtmiBvalidR {
        Usbcphy1OtgUtmiBvalidR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - usbcphy1_otg_utmi_linestate status bit"]
    #[inline(always)]
    pub fn usbcphy1_otg_utmi_linestate(&self) -> Usbcphy1OtgUtmiLinestateR {
        Usbcphy1OtgUtmiLinestateR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - usbcphy1_otg_utmi_hostdisconnect status"]
    #[inline(always)]
    pub fn usbcphy1_otg_utmi_hostdisconnect(&self) -> Usbcphy1OtgUtmiHostdisconnectR {
        Usbcphy1OtgUtmiHostdisconnectR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn usbcphy0_host_utmi_fs_xver_own(&self) -> Usbcphy0HostUtmiFsXverOwnR {
        Usbcphy0HostUtmiFsXverOwnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - usbcphy0_host_utmi_linestate status"]
    #[inline(always)]
    pub fn usbcphy0_host_utmi_linestate(&self) -> Usbcphy0HostUtmiLinestateR {
        Usbcphy0HostUtmiLinestateR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - usbcphy0_host_utmi_hostdisconnect status"]
    #[inline(always)]
    pub fn usbcphy0_host_utmi_hostdisconnect(&self) -> Usbcphy0HostUtmiHostdisconnectR {
        Usbcphy0HostUtmiHostdisconnectR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn usbcphy1_host_utmi_fs_xver_own(&self) -> Usbcphy1HostUtmiFsXverOwnR {
        Usbcphy1HostUtmiFsXverOwnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - usbcphy1_host_utmi_linestate status"]
    #[inline(always)]
    pub fn usbcphy1_host_utmi_linestate(&self) -> Usbcphy1HostUtmiLinestateR {
        Usbcphy1HostUtmiLinestateR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - usbcphy1_host_utmi_hostdisconnect status"]
    #[inline(always)]
    pub fn usbcphy1_host_utmi_hostdisconnect(&self) -> Usbcphy1HostUtmiHostdisconnectR {
        Usbcphy1HostUtmiHostdisconnectR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - usb20_phy0_stat_dp_attached status bit"]
    #[inline(always)]
    #[must_use]
    pub fn usb20_phy0_stat_dp_attached(&mut self) -> Usb20Phy0StatDpAttachedW<GrfSocStatus3Spec> {
        Usb20Phy0StatDpAttachedW::new(self, 0)
    }
    #[doc = "Bit 1 - usb20_phy0_stat_dcp_detected status bit"]
    #[inline(always)]
    #[must_use]
    pub fn usb20_phy0_stat_dcp_detected(&mut self) -> Usb20Phy0StatDcpDetectedW<GrfSocStatus3Spec> {
        Usb20Phy0StatDcpDetectedW::new(self, 1)
    }
    #[doc = "Bit 2 - usb20_phy0_stat_cp_detected status bit"]
    #[inline(always)]
    #[must_use]
    pub fn usb20_phy0_stat_cp_detected(&mut self) -> Usb20Phy0StatCpDetectedW<GrfSocStatus3Spec> {
        Usb20Phy0StatCpDetectedW::new(self, 2)
    }
    #[doc = "Bit 3 - usb20_phy1_stat_dp_attached status bit"]
    #[inline(always)]
    #[must_use]
    pub fn usb20_phy1_stat_dp_attached(&mut self) -> Usb20Phy1StatDpAttachedW<GrfSocStatus3Spec> {
        Usb20Phy1StatDpAttachedW::new(self, 3)
    }
    #[doc = "Bit 4 - usb20_phy1_stat_dcp_detected status bit"]
    #[inline(always)]
    #[must_use]
    pub fn usb20_phy1_stat_dcp_detected(&mut self) -> Usb20Phy1StatDcpDetectedW<GrfSocStatus3Spec> {
        Usb20Phy1StatDcpDetectedW::new(self, 4)
    }
    #[doc = "Bit 5 - usb20_phy1_stat_cp_detected status bit"]
    #[inline(always)]
    #[must_use]
    pub fn usb20_phy1_stat_cp_detected(&mut self) -> Usb20Phy1StatCpDetectedW<GrfSocStatus3Spec> {
        Usb20Phy1StatCpDetectedW::new(self, 5)
    }
    #[doc = "Bit 6 - usbcphy0_otg_utmi_sessend status bit"]
    #[inline(always)]
    #[must_use]
    pub fn usbcphy0_otg_utmi_sessend(&mut self) -> Usbcphy0OtgUtmiSessendW<GrfSocStatus3Spec> {
        Usbcphy0OtgUtmiSessendW::new(self, 6)
    }
    #[doc = "Bit 7 - usbcphy0_otg_utmi_avalid status bit"]
    #[inline(always)]
    #[must_use]
    pub fn usbcphy0_otg_utmi_avalid(&mut self) -> Usbcphy0OtgUtmiAvalidW<GrfSocStatus3Spec> {
        Usbcphy0OtgUtmiAvalidW::new(self, 7)
    }
    #[doc = "Bit 8 - usbcphy0_otg_utmi_iddig status bit"]
    #[inline(always)]
    #[must_use]
    pub fn usbcphy0_otg_utmi_iddig(&mut self) -> Usbcphy0OtgUtmiIddigW<GrfSocStatus3Spec> {
        Usbcphy0OtgUtmiIddigW::new(self, 8)
    }
    #[doc = "Bit 9 - usbcphy1_otg_utmi_sessend status bit"]
    #[inline(always)]
    #[must_use]
    pub fn usbcphy1_otg_utmi_sessend(&mut self) -> Usbcphy1OtgUtmiSessendW<GrfSocStatus3Spec> {
        Usbcphy1OtgUtmiSessendW::new(self, 9)
    }
    #[doc = "Bit 10 - usbcphy1_otg_utmi_avalid status bit"]
    #[inline(always)]
    #[must_use]
    pub fn usbcphy1_otg_utmi_avalid(&mut self) -> Usbcphy1OtgUtmiAvalidW<GrfSocStatus3Spec> {
        Usbcphy1OtgUtmiAvalidW::new(self, 10)
    }
    #[doc = "Bit 11 - usbcphy1_otg_utmi_iddig status bit"]
    #[inline(always)]
    #[must_use]
    pub fn usbcphy1_otg_utmi_iddig(&mut self) -> Usbcphy1OtgUtmiIddigW<GrfSocStatus3Spec> {
        Usbcphy1OtgUtmiIddigW::new(self, 11)
    }
}
#[doc = "SOC status register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_status3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_status3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfSocStatus3Spec;
impl crate::RegisterSpec for GrfSocStatus3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_soc_status3::R`](R) reader structure"]
impl crate::Readable for GrfSocStatus3Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_soc_status3::W`](W) writer structure"]
impl crate::Writable for GrfSocStatus3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_SOC_STATUS3 to value 0"]
impl crate::Resettable for GrfSocStatus3Spec {
    const RESET_VALUE: u32 = 0;
}
