#[doc = "Register `SOC_CON24` reader"]
pub type R = crate::R<SocCon24Spec>;
#[doc = "Register `SOC_CON24` writer"]
pub type W = crate::W<SocCon24Spec>;
#[doc = "Field `DPHY_TX1TX1_TURNREQUEST` reader - dphy_tx1tx1_turnrequest bit control"]
pub type DphyTx1tx1TurnrequestR = crate::FieldReader;
#[doc = "Field `DPHY_TX1TX1_TURNREQUEST` writer - dphy_tx1tx1_turnrequest bit control"]
pub type DphyTx1tx1TurnrequestW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DPHY_RX1_SRC_SEL` reader - dphy_rx1_src_sel bit control"]
pub type DphyRx1SrcSelR = crate::BitReader;
#[doc = "Field `DPHY_RX1_SRC_SEL` writer - dphy_rx1_src_sel bit control"]
pub type DphyRx1SrcSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPHY_TX1RX1_BASEDIR` reader - dphy_tx1rx1_basedir bit control"]
pub type DphyTx1rx1BasedirR = crate::BitReader;
#[doc = "Field `DPHY_TX1RX1_BASEDIR` writer - dphy_tx1rx1_basedir bit control"]
pub type DphyTx1rx1BasedirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPHY_TX1RX1_ENABLECLK` reader - dphy_tx1rx1_enableclk bit control"]
pub type DphyTx1rx1EnableclkR = crate::BitReader;
#[doc = "Field `DPHY_TX1RX1_ENABLECLK` writer - dphy_tx1rx1_enableclk bit control"]
pub type DphyTx1rx1EnableclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPHY_TX1RX1_MASTERSLAVEZ` reader - dphy_tx1rx1_masterslavez bit control"]
pub type DphyTx1rx1MasterslavezR = crate::BitReader;
#[doc = "Field `DPHY_TX1RX1_MASTERSLAVEZ` writer - dphy_tx1rx1_masterslavez bit control"]
pub type DphyTx1rx1MasterslavezW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "vopb_dsi_halt_sel bit control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VopbDsiHaltSel {
    #[doc = "0: mipi_dsi0_edpihalt"]
    D0 = 0,
    #[doc = "1: mipi_dsi0_edpihalt"]
    D1 = 1,
    #[doc = "2: low"]
    D2 = 2,
    #[doc = "3: high"]
    D3 = 3,
}
impl From<VopbDsiHaltSel> for u8 {
    #[inline(always)]
    fn from(variant: VopbDsiHaltSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VopbDsiHaltSel {
    type Ux = u8;
}
#[doc = "Field `VOPB_DSI_HALT_SEL` reader - vopb_dsi_halt_sel bit control"]
pub type VopbDsiHaltSelR = crate::FieldReader<VopbDsiHaltSel>;
impl VopbDsiHaltSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VopbDsiHaltSel {
        match self.bits {
            0 => VopbDsiHaltSel::D0,
            1 => VopbDsiHaltSel::D1,
            2 => VopbDsiHaltSel::D2,
            3 => VopbDsiHaltSel::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "mipi_dsi0_edpihalt"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == VopbDsiHaltSel::D0
    }
    #[doc = "mipi_dsi0_edpihalt"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == VopbDsiHaltSel::D1
    }
    #[doc = "low"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == VopbDsiHaltSel::D2
    }
    #[doc = "high"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == VopbDsiHaltSel::D3
    }
}
#[doc = "Field `VOPB_DSI_HALT_SEL` writer - vopb_dsi_halt_sel bit control"]
pub type VopbDsiHaltSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, VopbDsiHaltSel>;
impl<'a, REG> VopbDsiHaltSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "mipi_dsi0_edpihalt"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(VopbDsiHaltSel::D0)
    }
    #[doc = "mipi_dsi0_edpihalt"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(VopbDsiHaltSel::D1)
    }
    #[doc = "low"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(VopbDsiHaltSel::D2)
    }
    #[doc = "high"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(VopbDsiHaltSel::D3)
    }
}
#[doc = "vopl_dsi_halt_sel bit control\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VoplDsiHaltSel {
    #[doc = "0: mipi_dsi0_edpihalt"]
    D0 = 0,
    #[doc = "1: mipi_dsi0_edpihalt"]
    D1 = 1,
    #[doc = "2: low"]
    D2 = 2,
    #[doc = "3: high"]
    D3 = 3,
}
impl From<VoplDsiHaltSel> for u8 {
    #[inline(always)]
    fn from(variant: VoplDsiHaltSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VoplDsiHaltSel {
    type Ux = u8;
}
#[doc = "Field `VOPL_DSI_HALT_SEL` reader - vopl_dsi_halt_sel bit control"]
pub type VoplDsiHaltSelR = crate::FieldReader<VoplDsiHaltSel>;
impl VoplDsiHaltSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VoplDsiHaltSel {
        match self.bits {
            0 => VoplDsiHaltSel::D0,
            1 => VoplDsiHaltSel::D1,
            2 => VoplDsiHaltSel::D2,
            3 => VoplDsiHaltSel::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "mipi_dsi0_edpihalt"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == VoplDsiHaltSel::D0
    }
    #[doc = "mipi_dsi0_edpihalt"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == VoplDsiHaltSel::D1
    }
    #[doc = "low"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == VoplDsiHaltSel::D2
    }
    #[doc = "high"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == VoplDsiHaltSel::D3
    }
}
#[doc = "Field `VOPL_DSI_HALT_SEL` writer - vopl_dsi_halt_sel bit control"]
pub type VoplDsiHaltSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, VoplDsiHaltSel>;
impl<'a, REG> VoplDsiHaltSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "mipi_dsi0_edpihalt"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(VoplDsiHaltSel::D0)
    }
    #[doc = "mipi_dsi0_edpihalt"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(VoplDsiHaltSel::D1)
    }
    #[doc = "low"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(VoplDsiHaltSel::D2)
    }
    #[doc = "high"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(VoplDsiHaltSel::D3)
    }
}
#[doc = "vopb_dsi_ite_sel bit control\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VopbDsiIteSel {
    #[doc = "0: mipi_dsi0_edpite"]
    D0 = 0,
    #[doc = "1: mipi_dsi1_edpite"]
    D1 = 1,
    #[doc = "2: 0"]
    D2 = 2,
    #[doc = "3: 1"]
    D3 = 3,
}
impl From<VopbDsiIteSel> for u8 {
    #[inline(always)]
    fn from(variant: VopbDsiIteSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VopbDsiIteSel {
    type Ux = u8;
}
#[doc = "Field `VOPB_DSI_ITE_SEL` reader - vopb_dsi_ite_sel bit control"]
pub type VopbDsiIteSelR = crate::FieldReader<VopbDsiIteSel>;
impl VopbDsiIteSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VopbDsiIteSel {
        match self.bits {
            0 => VopbDsiIteSel::D0,
            1 => VopbDsiIteSel::D1,
            2 => VopbDsiIteSel::D2,
            3 => VopbDsiIteSel::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "mipi_dsi0_edpite"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == VopbDsiIteSel::D0
    }
    #[doc = "mipi_dsi1_edpite"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == VopbDsiIteSel::D1
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == VopbDsiIteSel::D2
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == VopbDsiIteSel::D3
    }
}
#[doc = "Field `VOPB_DSI_ITE_SEL` writer - vopb_dsi_ite_sel bit control"]
pub type VopbDsiIteSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, VopbDsiIteSel>;
impl<'a, REG> VopbDsiIteSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "mipi_dsi0_edpite"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(VopbDsiIteSel::D0)
    }
    #[doc = "mipi_dsi1_edpite"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(VopbDsiIteSel::D1)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(VopbDsiIteSel::D2)
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(VopbDsiIteSel::D3)
    }
}
#[doc = "vopl_dsi_ite_sel bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VoplDsiIteSel {
    #[doc = "0: mipi_dsi0_edpite"]
    D0 = 0,
    #[doc = "1: mipi_dsi1_edpite"]
    D1 = 1,
    #[doc = "2: 0"]
    D2 = 2,
    #[doc = "3: 1"]
    D3 = 3,
}
impl From<VoplDsiIteSel> for u8 {
    #[inline(always)]
    fn from(variant: VoplDsiIteSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VoplDsiIteSel {
    type Ux = u8;
}
#[doc = "Field `VOPL_DSI_ITE_SEL` reader - vopl_dsi_ite_sel bit control"]
pub type VoplDsiIteSelR = crate::FieldReader<VoplDsiIteSel>;
impl VoplDsiIteSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VoplDsiIteSel {
        match self.bits {
            0 => VoplDsiIteSel::D0,
            1 => VoplDsiIteSel::D1,
            2 => VoplDsiIteSel::D2,
            3 => VoplDsiIteSel::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "mipi_dsi0_edpite"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == VoplDsiIteSel::D0
    }
    #[doc = "mipi_dsi1_edpite"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == VoplDsiIteSel::D1
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == VoplDsiIteSel::D2
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == VoplDsiIteSel::D3
    }
}
#[doc = "Field `VOPL_DSI_ITE_SEL` writer - vopl_dsi_ite_sel bit control"]
pub type VoplDsiIteSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, VoplDsiIteSel>;
impl<'a, REG> VoplDsiIteSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "mipi_dsi0_edpite"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(VoplDsiIteSel::D0)
    }
    #[doc = "mipi_dsi1_edpite"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(VoplDsiIteSel::D1)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(VoplDsiIteSel::D2)
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(VoplDsiIteSel::D3)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - dphy_tx1tx1_turnrequest bit control"]
    #[inline(always)]
    pub fn dphy_tx1tx1_turnrequest(&self) -> DphyTx1tx1TurnrequestR {
        DphyTx1tx1TurnrequestR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - dphy_rx1_src_sel bit control"]
    #[inline(always)]
    pub fn dphy_rx1_src_sel(&self) -> DphyRx1SrcSelR {
        DphyRx1SrcSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - dphy_tx1rx1_basedir bit control"]
    #[inline(always)]
    pub fn dphy_tx1rx1_basedir(&self) -> DphyTx1rx1BasedirR {
        DphyTx1rx1BasedirR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - dphy_tx1rx1_enableclk bit control"]
    #[inline(always)]
    pub fn dphy_tx1rx1_enableclk(&self) -> DphyTx1rx1EnableclkR {
        DphyTx1rx1EnableclkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - dphy_tx1rx1_masterslavez bit control"]
    #[inline(always)]
    pub fn dphy_tx1rx1_masterslavez(&self) -> DphyTx1rx1MasterslavezR {
        DphyTx1rx1MasterslavezR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - vopb_dsi_halt_sel bit control"]
    #[inline(always)]
    pub fn vopb_dsi_halt_sel(&self) -> VopbDsiHaltSelR {
        VopbDsiHaltSelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - vopl_dsi_halt_sel bit control"]
    #[inline(always)]
    pub fn vopl_dsi_halt_sel(&self) -> VoplDsiHaltSelR {
        VoplDsiHaltSelR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - vopb_dsi_ite_sel bit control"]
    #[inline(always)]
    pub fn vopb_dsi_ite_sel(&self) -> VopbDsiIteSelR {
        VopbDsiIteSelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - vopl_dsi_ite_sel bit control"]
    #[inline(always)]
    pub fn vopl_dsi_ite_sel(&self) -> VoplDsiIteSelR {
        VoplDsiIteSelR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - dphy_tx1tx1_turnrequest bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_tx1tx1_turnrequest(&mut self) -> DphyTx1tx1TurnrequestW<SocCon24Spec> {
        DphyTx1tx1TurnrequestW::new(self, 0)
    }
    #[doc = "Bit 4 - dphy_rx1_src_sel bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_rx1_src_sel(&mut self) -> DphyRx1SrcSelW<SocCon24Spec> {
        DphyRx1SrcSelW::new(self, 4)
    }
    #[doc = "Bit 5 - dphy_tx1rx1_basedir bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_tx1rx1_basedir(&mut self) -> DphyTx1rx1BasedirW<SocCon24Spec> {
        DphyTx1rx1BasedirW::new(self, 5)
    }
    #[doc = "Bit 6 - dphy_tx1rx1_enableclk bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_tx1rx1_enableclk(&mut self) -> DphyTx1rx1EnableclkW<SocCon24Spec> {
        DphyTx1rx1EnableclkW::new(self, 6)
    }
    #[doc = "Bit 7 - dphy_tx1rx1_masterslavez bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_tx1rx1_masterslavez(&mut self) -> DphyTx1rx1MasterslavezW<SocCon24Spec> {
        DphyTx1rx1MasterslavezW::new(self, 7)
    }
    #[doc = "Bits 8:9 - vopb_dsi_halt_sel bit control"]
    #[inline(always)]
    #[must_use]
    pub fn vopb_dsi_halt_sel(&mut self) -> VopbDsiHaltSelW<SocCon24Spec> {
        VopbDsiHaltSelW::new(self, 8)
    }
    #[doc = "Bits 10:11 - vopl_dsi_halt_sel bit control"]
    #[inline(always)]
    #[must_use]
    pub fn vopl_dsi_halt_sel(&mut self) -> VoplDsiHaltSelW<SocCon24Spec> {
        VoplDsiHaltSelW::new(self, 10)
    }
    #[doc = "Bits 12:13 - vopb_dsi_ite_sel bit control"]
    #[inline(always)]
    #[must_use]
    pub fn vopb_dsi_ite_sel(&mut self) -> VopbDsiIteSelW<SocCon24Spec> {
        VopbDsiIteSelW::new(self, 12)
    }
    #[doc = "Bits 14:15 - vopl_dsi_ite_sel bit control"]
    #[inline(always)]
    #[must_use]
    pub fn vopl_dsi_ite_sel(&mut self) -> VoplDsiIteSelW<SocCon24Spec> {
        VoplDsiIteSelW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<SocCon24Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "SoC control register 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con24::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con24::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SocCon24Spec;
impl crate::RegisterSpec for SocCon24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soc_con24::R`](R) reader structure"]
impl crate::Readable for SocCon24Spec {}
#[doc = "`write(|w| ..)` method takes [`soc_con24::W`](W) writer structure"]
impl crate::Writable for SocCon24Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOC_CON24 to value 0x39f0"]
impl crate::Resettable for SocCon24Spec {
    const RESET_VALUE: u32 = 0x39f0;
}
