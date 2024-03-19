#[doc = "Register `GRF_USB3PHY1_CON1` reader"]
pub type R = crate::R<GrfUsb3phy1Con1Spec>;
#[doc = "Register `GRF_USB3PHY1_CON1` writer"]
pub type W = crate::W<GrfUsb3phy1Con1Spec>;
#[doc = "pipe interface select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PipeSel {
    #[doc = "0: select pipe interface from usb3otg"]
    B0 = 0,
    #[doc = "1: select pipe interface from grf controller register"]
    B1 = 1,
}
impl From<PipeSel> for bool {
    #[inline(always)]
    fn from(variant: PipeSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE_SEL` reader - pipe interface select"]
pub type PipeSelR = crate::BitReader<PipeSel>;
impl PipeSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PipeSel {
        match self.bits {
            false => PipeSel::B0,
            true => PipeSel::B1,
        }
    }
    #[doc = "select pipe interface from usb3otg"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PipeSel::B0
    }
    #[doc = "select pipe interface from grf controller register"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PipeSel::B1
    }
}
#[doc = "Field `PIPE_SEL` writer - pipe interface select"]
pub type PipeSelW<'a, REG> = crate::BitWriter<'a, REG, PipeSel>;
impl<'a, REG> PipeSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "select pipe interface from usb3otg"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PipeSel::B0)
    }
    #[doc = "select pipe interface from grf controller register"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PipeSel::B1)
    }
}
#[doc = "Field `RXTERMINATION` reader - rx termination\n\npipe_sel=1, select this bit to TypeC PHY"]
pub type RxterminationR = crate::BitReader;
#[doc = "Field `RXTERMINATION` writer - rx termination\n\npipe_sel=1, select this bit to TypeC PHY"]
pub type RxterminationW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXELECIDLE` reader - txelecidle\n\npipe_sel=1, select this bit to TypeC PHY"]
pub type TxelecidleR = crate::BitReader;
#[doc = "Field `TXELECIDLE` writer - txelecidle\n\npipe_sel=1, select this bit to TypeC PHY"]
pub type TxelecidleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POWERDOWN` reader - powerdown\n\npipe_sel: select this two bit to TypeC PHY"]
pub type PowerdownR = crate::FieldReader;
#[doc = "Field `POWERDOWN` writer - powerdown\n\npipe_sel: select this two bit to TypeC PHY"]
pub type PowerdownW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TXDETECTRXLOOPBK` reader - txdetectrxloopbk\n\npipe_sel=1, select this bit to TypeC PHY"]
pub type TxdetectrxloopbkR = crate::BitReader;
#[doc = "Field `TXDETECTRXLOOPBK` writer - txdetectrxloopbk\n\npipe_sel=1, select this bit to TypeC PHY"]
pub type TxdetectrxloopbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCDHALTONRESET` reader - OCDHaltOnReset\n\nTCPC extensa core JTAG OCDHaltOnReset"]
pub type OcdhaltonresetR = crate::BitReader;
#[doc = "Field `OCDHALTONRESET` writer - OCDHaltOnReset\n\nTCPC extensa core JTAG OCDHaltOnReset"]
pub type OcdhaltonresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRRESET` reader - BRreset\n\nTCPC extensa core JTAG BRreset"]
pub type BrresetR = crate::BitReader;
#[doc = "Field `BRRESET` writer - BRreset\n\nTCPC extensa core JTAG BRreset"]
pub type BrresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRESET` reader - DReset\n\nTCPC extensa core JTAG DReset"]
pub type DresetR = crate::BitReader;
#[doc = "Field `DRESET` writer - DReset\n\nTCPC extensa core JTAG DReset"]
pub type DresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTRST` reader - JTRST\n\nTCPC extensa core JTAG JTRST reset control"]
pub type JtrstR = crate::BitReader;
#[doc = "Field `JTRST` writer - JTRST\n\nTCPC extensa core JTAG JTRST reset control"]
pub type JtrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "vbus overvoltage\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VbusOvervoltageN {
    #[doc = "0: vbus over voltage"]
    B0 = 0,
    #[doc = "1: vbus not over voltage"]
    B1 = 1,
}
impl From<VbusOvervoltageN> for bool {
    #[inline(always)]
    fn from(variant: VbusOvervoltageN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBUS_OVERVOLTAGE_N` reader - vbus overvoltage"]
pub type VbusOvervoltageNR = crate::BitReader<VbusOvervoltageN>;
impl VbusOvervoltageNR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VbusOvervoltageN {
        match self.bits {
            false => VbusOvervoltageN::B0,
            true => VbusOvervoltageN::B1,
        }
    }
    #[doc = "vbus over voltage"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VbusOvervoltageN::B0
    }
    #[doc = "vbus not over voltage"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VbusOvervoltageN::B1
    }
}
#[doc = "Field `VBUS_OVERVOLTAGE_N` writer - vbus overvoltage"]
pub type VbusOvervoltageNW<'a, REG> = crate::BitWriter<'a, REG, VbusOvervoltageN>;
impl<'a, REG> VbusOvervoltageNW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "vbus over voltage"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VbusOvervoltageN::B0)
    }
    #[doc = "vbus not over voltage"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VbusOvervoltageN::B1)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - pipe interface select"]
    #[inline(always)]
    pub fn pipe_sel(&self) -> PipeSelR {
        PipeSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - rx termination\n\npipe_sel=1, select this bit to TypeC PHY"]
    #[inline(always)]
    pub fn rxtermination(&self) -> RxterminationR {
        RxterminationR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - txelecidle\n\npipe_sel=1, select this bit to TypeC PHY"]
    #[inline(always)]
    pub fn txelecidle(&self) -> TxelecidleR {
        TxelecidleR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - powerdown\n\npipe_sel: select this two bit to TypeC PHY"]
    #[inline(always)]
    pub fn powerdown(&self) -> PowerdownR {
        PowerdownR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - txdetectrxloopbk\n\npipe_sel=1, select this bit to TypeC PHY"]
    #[inline(always)]
    pub fn txdetectrxloopbk(&self) -> TxdetectrxloopbkR {
        TxdetectrxloopbkR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - OCDHaltOnReset\n\nTCPC extensa core JTAG OCDHaltOnReset"]
    #[inline(always)]
    pub fn ocdhaltonreset(&self) -> OcdhaltonresetR {
        OcdhaltonresetR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BRreset\n\nTCPC extensa core JTAG BRreset"]
    #[inline(always)]
    pub fn brreset(&self) -> BrresetR {
        BrresetR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DReset\n\nTCPC extensa core JTAG DReset"]
    #[inline(always)]
    pub fn dreset(&self) -> DresetR {
        DresetR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - JTRST\n\nTCPC extensa core JTAG JTRST reset control"]
    #[inline(always)]
    pub fn jtrst(&self) -> JtrstR {
        JtrstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - vbus overvoltage"]
    #[inline(always)]
    pub fn vbus_overvoltage_n(&self) -> VbusOvervoltageNR {
        VbusOvervoltageNR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - pipe interface select"]
    #[inline(always)]
    #[must_use]
    pub fn pipe_sel(&mut self) -> PipeSelW<GrfUsb3phy1Con1Spec> {
        PipeSelW::new(self, 0)
    }
    #[doc = "Bit 1 - rx termination\n\npipe_sel=1, select this bit to TypeC PHY"]
    #[inline(always)]
    #[must_use]
    pub fn rxtermination(&mut self) -> RxterminationW<GrfUsb3phy1Con1Spec> {
        RxterminationW::new(self, 1)
    }
    #[doc = "Bit 2 - txelecidle\n\npipe_sel=1, select this bit to TypeC PHY"]
    #[inline(always)]
    #[must_use]
    pub fn txelecidle(&mut self) -> TxelecidleW<GrfUsb3phy1Con1Spec> {
        TxelecidleW::new(self, 2)
    }
    #[doc = "Bits 3:4 - powerdown\n\npipe_sel: select this two bit to TypeC PHY"]
    #[inline(always)]
    #[must_use]
    pub fn powerdown(&mut self) -> PowerdownW<GrfUsb3phy1Con1Spec> {
        PowerdownW::new(self, 3)
    }
    #[doc = "Bit 5 - txdetectrxloopbk\n\npipe_sel=1, select this bit to TypeC PHY"]
    #[inline(always)]
    #[must_use]
    pub fn txdetectrxloopbk(&mut self) -> TxdetectrxloopbkW<GrfUsb3phy1Con1Spec> {
        TxdetectrxloopbkW::new(self, 5)
    }
    #[doc = "Bit 8 - OCDHaltOnReset\n\nTCPC extensa core JTAG OCDHaltOnReset"]
    #[inline(always)]
    #[must_use]
    pub fn ocdhaltonreset(&mut self) -> OcdhaltonresetW<GrfUsb3phy1Con1Spec> {
        OcdhaltonresetW::new(self, 8)
    }
    #[doc = "Bit 9 - BRreset\n\nTCPC extensa core JTAG BRreset"]
    #[inline(always)]
    #[must_use]
    pub fn brreset(&mut self) -> BrresetW<GrfUsb3phy1Con1Spec> {
        BrresetW::new(self, 9)
    }
    #[doc = "Bit 10 - DReset\n\nTCPC extensa core JTAG DReset"]
    #[inline(always)]
    #[must_use]
    pub fn dreset(&mut self) -> DresetW<GrfUsb3phy1Con1Spec> {
        DresetW::new(self, 10)
    }
    #[doc = "Bit 11 - JTRST\n\nTCPC extensa core JTAG JTRST reset control"]
    #[inline(always)]
    #[must_use]
    pub fn jtrst(&mut self) -> JtrstW<GrfUsb3phy1Con1Spec> {
        JtrstW::new(self, 11)
    }
    #[doc = "Bit 12 - vbus overvoltage"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_overvoltage_n(&mut self) -> VbusOvervoltageNW<GrfUsb3phy1Con1Spec> {
        VbusOvervoltageNW::new(self, 12)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfUsb3phy1Con1Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "TypeC PHY/TCPD PHY/TCPC Control register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3phy1_con1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3phy1_con1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfUsb3phy1Con1Spec;
impl crate::RegisterSpec for GrfUsb3phy1Con1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_usb3phy1_con1::R`](R) reader structure"]
impl crate::Readable for GrfUsb3phy1Con1Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_usb3phy1_con1::W`](W) writer structure"]
impl crate::Writable for GrfUsb3phy1Con1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_USB3PHY1_CON1 to value 0x1000"]
impl crate::Resettable for GrfUsb3phy1Con1Spec {
    const RESET_VALUE: u32 = 0x1000;
}
