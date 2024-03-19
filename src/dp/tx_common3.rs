#[doc = "Register `TX_COMMON3` reader"]
pub type R = crate::R<TxCommon3Spec>;
#[doc = "Register `TX_COMMON3` writer"]
pub type W = crate::W<TxCommon3Spec>;
#[doc = "ch0 select i_ref_clk_24m for scan\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScanClkSel {
    #[doc = "0: select tx_bscan_data&lt;0>"]
    B0 = 0,
    #[doc = "1: select i_ref_clk_24m"]
    B1 = 1,
}
impl From<ScanClkSel> for bool {
    #[inline(always)]
    fn from(variant: ScanClkSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCAN_CLK_SEL` reader - ch0 select i_ref_clk_24m for scan"]
pub type ScanClkSelR = crate::BitReader<ScanClkSel>;
impl ScanClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ScanClkSel {
        match self.bits {
            false => ScanClkSel::B0,
            true => ScanClkSel::B1,
        }
    }
    #[doc = "select tx_bscan_data&lt;0>"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ScanClkSel::B0
    }
    #[doc = "select i_ref_clk_24m"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ScanClkSel::B1
    }
}
#[doc = "Field `SCAN_CLK_SEL` writer - ch0 select i_ref_clk_24m for scan"]
pub type ScanClkSelW<'a, REG> = crate::BitWriter1C<'a, REG, ScanClkSel>;
impl<'a, REG> ScanClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "select tx_bscan_data&lt;0>"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ScanClkSel::B0)
    }
    #[doc = "select i_ref_clk_24m"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ScanClkSel::B1)
    }
}
#[doc = "TX input clock inverse enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkInverseEn {
    #[doc = "0: normal"]
    B0 = 0,
    #[doc = "1: TX input clock inverse"]
    B1 = 1,
}
impl From<ClkInverseEn> for bool {
    #[inline(always)]
    fn from(variant: ClkInverseEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_INVERSE_EN` reader - TX input clock inverse enable"]
pub type ClkInverseEnR = crate::BitReader<ClkInverseEn>;
impl ClkInverseEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkInverseEn {
        match self.bits {
            false => ClkInverseEn::B0,
            true => ClkInverseEn::B1,
        }
    }
    #[doc = "normal"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkInverseEn::B0
    }
    #[doc = "TX input clock inverse"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkInverseEn::B1
    }
}
#[doc = "Field `CLK_INVERSE_EN` writer - TX input clock inverse enable"]
pub type ClkInverseEnW<'a, REG> = crate::BitWriter1C<'a, REG, ClkInverseEn>;
impl<'a, REG> ClkInverseEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkInverseEn::B0)
    }
    #[doc = "TX input clock inverse"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkInverseEn::B1)
    }
}
#[doc = "Select /20 clock delay (clk_div2_ssc &amp; \n\ntx_txd_clk)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkDlySel {
    #[doc = "0: delay=150ps"]
    B00000 = 0,
    #[doc = "1: delay=150ps+1*70ps"]
    B00001 = 1,
    #[doc = "2: delay=150ps+2*70ps"]
    B00010 = 2,
    #[doc = "3: delay=150ps+3*70ps . . . . . ."]
    B00011 = 3,
}
impl From<ClkDlySel> for u8 {
    #[inline(always)]
    fn from(variant: ClkDlySel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkDlySel {
    type Ux = u8;
}
#[doc = "Field `CLK_DLY_SEL` reader - Select /20 clock delay (clk_div2_ssc &amp; \n\ntx_txd_clk)"]
pub type ClkDlySelR = crate::FieldReader<ClkDlySel>;
impl ClkDlySelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkDlySel> {
        match self.bits {
            0 => Some(ClkDlySel::B00000),
            1 => Some(ClkDlySel::B00001),
            2 => Some(ClkDlySel::B00010),
            3 => Some(ClkDlySel::B00011),
            _ => None,
        }
    }
    #[doc = "delay=150ps"]
    #[inline(always)]
    pub fn is_b00000(&self) -> bool {
        *self == ClkDlySel::B00000
    }
    #[doc = "delay=150ps+1*70ps"]
    #[inline(always)]
    pub fn is_b00001(&self) -> bool {
        *self == ClkDlySel::B00001
    }
    #[doc = "delay=150ps+2*70ps"]
    #[inline(always)]
    pub fn is_b00010(&self) -> bool {
        *self == ClkDlySel::B00010
    }
    #[doc = "delay=150ps+3*70ps . . . . . ."]
    #[inline(always)]
    pub fn is_b00011(&self) -> bool {
        *self == ClkDlySel::B00011
    }
}
#[doc = "Field `CLK_DLY_SEL` writer - Select /20 clock delay (clk_div2_ssc &amp; \n\ntx_txd_clk)"]
pub type ClkDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 5, ClkDlySel>;
impl<'a, REG> ClkDlySelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "delay=150ps"]
    #[inline(always)]
    pub fn b00000(self) -> &'a mut crate::W<REG> {
        self.variant(ClkDlySel::B00000)
    }
    #[doc = "delay=150ps+1*70ps"]
    #[inline(always)]
    pub fn b00001(self) -> &'a mut crate::W<REG> {
        self.variant(ClkDlySel::B00001)
    }
    #[doc = "delay=150ps+2*70ps"]
    #[inline(always)]
    pub fn b00010(self) -> &'a mut crate::W<REG> {
        self.variant(ClkDlySel::B00010)
    }
    #[doc = "delay=150ps+3*70ps . . . . . ."]
    #[inline(always)]
    pub fn b00011(self) -> &'a mut crate::W<REG> {
        self.variant(ClkDlySel::B00011)
    }
}
impl R {
    #[doc = "Bit 1 - ch0 select i_ref_clk_24m for scan"]
    #[inline(always)]
    pub fn scan_clk_sel(&self) -> ScanClkSelR {
        ScanClkSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX input clock inverse enable"]
    #[inline(always)]
    pub fn clk_inverse_en(&self) -> ClkInverseEnR {
        ClkInverseEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - Select /20 clock delay (clk_div2_ssc &amp; \n\ntx_txd_clk)"]
    #[inline(always)]
    pub fn clk_dly_sel(&self) -> ClkDlySelR {
        ClkDlySelR::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - ch0 select i_ref_clk_24m for scan"]
    #[inline(always)]
    #[must_use]
    pub fn scan_clk_sel(&mut self) -> ScanClkSelW<TxCommon3Spec> {
        ScanClkSelW::new(self, 1)
    }
    #[doc = "Bit 2 - TX input clock inverse enable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_inverse_en(&mut self) -> ClkInverseEnW<TxCommon3Spec> {
        ClkInverseEnW::new(self, 2)
    }
    #[doc = "Bits 3:7 - Select /20 clock delay (clk_div2_ssc &amp; \n\ntx_txd_clk)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_dly_sel(&mut self) -> ClkDlySelW<TxCommon3Spec> {
        ClkDlySelW::new(self, 3)
    }
}
#[doc = "Tx terminal resistor control3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_common3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_common3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxCommon3Spec;
impl crate::RegisterSpec for TxCommon3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_common3::R`](R) reader structure"]
impl crate::Readable for TxCommon3Spec {}
#[doc = "`write(|w| ..)` method takes [`tx_common3::W`](W) writer structure"]
impl crate::Writable for TxCommon3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xfe;
}
#[doc = "`reset()` method sets TX_COMMON3 to value 0"]
impl crate::Resettable for TxCommon3Spec {
    const RESET_VALUE: u32 = 0;
}
