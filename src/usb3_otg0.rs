#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0xc100],
    usb3_gsbuscfg0: Usb3Gsbuscfg0,
    usb3_gsbuscfg1: Usb3Gsbuscfg1,
    usb3_gtxthrcfg: Usb3Gtxthrcfg,
    usb3_grxthrcfg: Usb3Grxthrcfg,
    usb3_gctl: Usb3Gctl,
    usb3_gpmsts: Usb3Gpmsts,
    usb3_gsts: Usb3Gsts,
    usb3_guctl1: Usb3Guctl1,
    usb3_gsnpsid: Usb3Gsnpsid,
    usb3_ggpio: Usb3Ggpio,
    usb3_guid: Usb3Guid,
    usb3_guctl: Usb3Guctl,
    usb3_gbuserraddrlo: Usb3Gbuserraddrlo,
    usb3_gbuserraddrhi: Usb3Gbuserraddrhi,
    usb3_gprtbimaplo: Usb3Gprtbimaplo,
    _reserved15: [u8; 0x04],
    usb3_ghwparams0: Usb3Ghwparams0,
    usb3_ghwparams1: Usb3Ghwparams1,
    usb3_ghwparams2: Usb3Ghwparams2,
    usb3_ghwparams3: Usb3Ghwparams3,
    usb3_ghwparams4: Usb3Ghwparams4,
    usb3_ghwparams5: Usb3Ghwparams5,
    usb3_ghwparams6: Usb3Ghwparams6,
    usb3_ghwparams7: Usb3Ghwparams7,
    usb3_gdbgfifospace: Usb3Gdbgfifospace,
    usb3_gdbgltssm: Usb3Gdbgltssm,
    usb3_gdbglnmcc: Usb3Gdbglnmcc,
    usb3_gdbgbmu: Usb3Gdbgbmu,
    usb3_gdbglspmux: Usb3Gdbglspmux,
    usb3_gdbglsp: Usb3Gdbglsp,
    usb3_gdbgepinfo0: Usb3Gdbgepinfo0,
    usb3_gdbgepinfo1: Usb3Gdbgepinfo1,
    usb3_gprtbimap_hslo: Usb3GprtbimapHslo,
    _reserved32: [u8; 0x04],
    usb3_gprtbimap_fslo: Usb3GprtbimapFslo,
    _reserved33: [u8; 0x74],
    usb3_gusb2phycfg0: Usb3Gusb2phycfg0,
    _reserved34: [u8; 0xbc],
    usb3_gusb3pipectl0: Usb3Gusb3pipectl0,
    _reserved35: [u8; 0x3c],
    usb3_gtxfifosiz: [Usb3Gtxfifosiz; 7],
    _reserved36: [u8; 0x64],
    usb3_grxfifosiz: [Usb3Grxfifosiz; 3],
    _reserved37: [u8; 0x74],
    usb3_gevntadrlo0: Usb3Gevntadrlo0,
    usb3_gevntadrhi0: Usb3Gevntadrhi0,
    usb3_gevntsiz0: Usb3Gevntsiz0,
    usb3_gevntcount0: Usb3Gevntcount0,
    _reserved41: [u8; 0x01f0],
    usb3_ghwparams8: Usb3Ghwparams8,
    _reserved42: [u8; 0x0c],
    usb3_gtxfifopridev: Usb3Gtxfifopridev,
    _reserved43: [u8; 0x04],
    usb3_gtxfifoprihst: Usb3Gtxfifoprihst,
    usb3_grxfifoprihst: Usb3Grxfifoprihst,
    usb3_gfifopridbc: Usb3Gfifopridbc,
    usb3_gdmahlratio: Usb3Gdmahlratio,
    _reserved47: [u8; 0x08],
    usb3_gfladj: Usb3Gfladj,
    _reserved48: [u8; 0xcc],
    usb3_dcfg: Usb3Dcfg,
    usb3_dctl: Usb3Dctl,
    usb3_devten: Usb3Devten,
    usb3_dsts: Usb3Dsts,
    usb3_dgcmdpar: Usb3Dgcmdpar,
    usb3_dgcmd: Usb3Dgcmd,
    _reserved54: [u8; 0x08],
    usb3_dalepena: Usb3Dalepena,
    _reserved55: [u8; 0xdc],
    usb3_depcmdpar2: (),
    _reserved56: [u8; 0x04],
    usb3_depcmdpar1: (),
    _reserved57: [u8; 0x04],
    usb3_depcmdpar0: (),
    _reserved58: [u8; 0x04],
    usb3_depcmd: (),
}
impl RegisterBlock {
    #[doc = "0xc100 - Global SoC Bus Configuration Register 0"]
    #[inline(always)]
    pub const fn usb3_gsbuscfg0(&self) -> &Usb3Gsbuscfg0 {
        &self.usb3_gsbuscfg0
    }
    #[doc = "0xc104 - Global SoC Bus Configuration Register 1"]
    #[inline(always)]
    pub const fn usb3_gsbuscfg1(&self) -> &Usb3Gsbuscfg1 {
        &self.usb3_gsbuscfg1
    }
    #[doc = "0xc108 - Global Tx Threshold Control Register"]
    #[inline(always)]
    pub const fn usb3_gtxthrcfg(&self) -> &Usb3Gtxthrcfg {
        &self.usb3_gtxthrcfg
    }
    #[doc = "0xc10c - Global Rx Threshold Control Register"]
    #[inline(always)]
    pub const fn usb3_grxthrcfg(&self) -> &Usb3Grxthrcfg {
        &self.usb3_grxthrcfg
    }
    #[doc = "0xc110 - Global Core Control Register"]
    #[inline(always)]
    pub const fn usb3_gctl(&self) -> &Usb3Gctl {
        &self.usb3_gctl
    }
    #[doc = "0xc114 - Global Power Management Status Register"]
    #[inline(always)]
    pub const fn usb3_gpmsts(&self) -> &Usb3Gpmsts {
        &self.usb3_gpmsts
    }
    #[doc = "0xc118 - Global Status Register"]
    #[inline(always)]
    pub const fn usb3_gsts(&self) -> &Usb3Gsts {
        &self.usb3_gsts
    }
    #[doc = "0xc11c - Global User Control Register 1"]
    #[inline(always)]
    pub const fn usb3_guctl1(&self) -> &Usb3Guctl1 {
        &self.usb3_guctl1
    }
    #[doc = "0xc120 - Global SNPS ID Register"]
    #[inline(always)]
    pub const fn usb3_gsnpsid(&self) -> &Usb3Gsnpsid {
        &self.usb3_gsnpsid
    }
    #[doc = "0xc124 - Global General Purpose Input/Output Register"]
    #[inline(always)]
    pub const fn usb3_ggpio(&self) -> &Usb3Ggpio {
        &self.usb3_ggpio
    }
    #[doc = "0xc128 - Global User ID Register"]
    #[inline(always)]
    pub const fn usb3_guid(&self) -> &Usb3Guid {
        &self.usb3_guid
    }
    #[doc = "0xc12c - Global User Control Register"]
    #[inline(always)]
    pub const fn usb3_guctl(&self) -> &Usb3Guctl {
        &self.usb3_guctl
    }
    #[doc = "0xc130 - Global SoC Bus Error Address Register - Low"]
    #[inline(always)]
    pub const fn usb3_gbuserraddrlo(&self) -> &Usb3Gbuserraddrlo {
        &self.usb3_gbuserraddrlo
    }
    #[doc = "0xc134 - Global SoC Bus Error Address Register - High"]
    #[inline(always)]
    pub const fn usb3_gbuserraddrhi(&self) -> &Usb3Gbuserraddrhi {
        &self.usb3_gbuserraddrhi
    }
    #[doc = "0xc138 - Global SS Port to Bus Instance Mapping Register - Low"]
    #[inline(always)]
    pub const fn usb3_gprtbimaplo(&self) -> &Usb3Gprtbimaplo {
        &self.usb3_gprtbimaplo
    }
    #[doc = "0xc140 - Global Hardware Parameters Register 0"]
    #[inline(always)]
    pub const fn usb3_ghwparams0(&self) -> &Usb3Ghwparams0 {
        &self.usb3_ghwparams0
    }
    #[doc = "0xc144 - Global Hardware Parameters Register 1"]
    #[inline(always)]
    pub const fn usb3_ghwparams1(&self) -> &Usb3Ghwparams1 {
        &self.usb3_ghwparams1
    }
    #[doc = "0xc148 - Global Hardware Parameters Register 2"]
    #[inline(always)]
    pub const fn usb3_ghwparams2(&self) -> &Usb3Ghwparams2 {
        &self.usb3_ghwparams2
    }
    #[doc = "0xc14c - Global Hardware Parameters Register 3"]
    #[inline(always)]
    pub const fn usb3_ghwparams3(&self) -> &Usb3Ghwparams3 {
        &self.usb3_ghwparams3
    }
    #[doc = "0xc150 - Global Hardware Parameters Register 4"]
    #[inline(always)]
    pub const fn usb3_ghwparams4(&self) -> &Usb3Ghwparams4 {
        &self.usb3_ghwparams4
    }
    #[doc = "0xc154 - Global Hardware Parameters Register 5"]
    #[inline(always)]
    pub const fn usb3_ghwparams5(&self) -> &Usb3Ghwparams5 {
        &self.usb3_ghwparams5
    }
    #[doc = "0xc158 - Global Hardware Parameters Register 6"]
    #[inline(always)]
    pub const fn usb3_ghwparams6(&self) -> &Usb3Ghwparams6 {
        &self.usb3_ghwparams6
    }
    #[doc = "0xc15c - Global Hardware Parameters Register 7"]
    #[inline(always)]
    pub const fn usb3_ghwparams7(&self) -> &Usb3Ghwparams7 {
        &self.usb3_ghwparams7
    }
    #[doc = "0xc160 - Global Debug Queue/FIFO Space Available Register"]
    #[inline(always)]
    pub const fn usb3_gdbgfifospace(&self) -> &Usb3Gdbgfifospace {
        &self.usb3_gdbgfifospace
    }
    #[doc = "0xc164 - Global Debug LTSSM Register"]
    #[inline(always)]
    pub const fn usb3_gdbgltssm(&self) -> &Usb3Gdbgltssm {
        &self.usb3_gdbgltssm
    }
    #[doc = "0xc168 - Global Debug LNMCC Register"]
    #[inline(always)]
    pub const fn usb3_gdbglnmcc(&self) -> &Usb3Gdbglnmcc {
        &self.usb3_gdbglnmcc
    }
    #[doc = "0xc16c - Global Debug BMU Register"]
    #[inline(always)]
    pub const fn usb3_gdbgbmu(&self) -> &Usb3Gdbgbmu {
        &self.usb3_gdbgbmu
    }
    #[doc = "0xc170 - Global Debug LSP MUX Register - Device"]
    #[inline(always)]
    pub const fn usb3_gdbglspmux(&self) -> &Usb3Gdbglspmux {
        &self.usb3_gdbglspmux
    }
    #[doc = "0xc174 - Global Debug LSP Register"]
    #[inline(always)]
    pub const fn usb3_gdbglsp(&self) -> &Usb3Gdbglsp {
        &self.usb3_gdbglsp
    }
    #[doc = "0xc178 - Global Debug Endpoint Information Register 0"]
    #[inline(always)]
    pub const fn usb3_gdbgepinfo0(&self) -> &Usb3Gdbgepinfo0 {
        &self.usb3_gdbgepinfo0
    }
    #[doc = "0xc17c - Global Debug Endpoint Information Register 1"]
    #[inline(always)]
    pub const fn usb3_gdbgepinfo1(&self) -> &Usb3Gdbgepinfo1 {
        &self.usb3_gdbgepinfo1
    }
    #[doc = "0xc180 - Global High-Speed Port to Bus Instance Mapping Register - Low"]
    #[inline(always)]
    pub const fn usb3_gprtbimap_hslo(&self) -> &Usb3GprtbimapHslo {
        &self.usb3_gprtbimap_hslo
    }
    #[doc = "0xc188 - Global Full-Speed Port to Bus Instance Mapping Register - Low"]
    #[inline(always)]
    pub const fn usb3_gprtbimap_fslo(&self) -> &Usb3GprtbimapFslo {
        &self.usb3_gprtbimap_fslo
    }
    #[doc = "0xc200 - Global USB2 PHY Configuration Register 0"]
    #[inline(always)]
    pub const fn usb3_gusb2phycfg0(&self) -> &Usb3Gusb2phycfg0 {
        &self.usb3_gusb2phycfg0
    }
    #[doc = "0xc2c0 - Global USB3 PIPE Control Register 0"]
    #[inline(always)]
    pub const fn usb3_gusb3pipectl0(&self) -> &Usb3Gusb3pipectl0 {
        &self.usb3_gusb3pipectl0
    }
    #[doc = "0xc300..0xc31c - Global Transmit FIFO Size Register n"]
    #[inline(always)]
    pub const fn usb3_gtxfifosiz(&self, n: usize) -> &Usb3Gtxfifosiz {
        &self.usb3_gtxfifosiz[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc300..0xc31c - Global Transmit FIFO Size Register n"]
    #[inline(always)]
    pub fn usb3_gtxfifosiz_iter(&self) -> impl Iterator<Item = &Usb3Gtxfifosiz> {
        self.usb3_gtxfifosiz.iter()
    }
    #[doc = "0xc380..0xc38c - Global Receive FIFO Size Register n"]
    #[inline(always)]
    pub const fn usb3_grxfifosiz(&self, n: usize) -> &Usb3Grxfifosiz {
        &self.usb3_grxfifosiz[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc380..0xc38c - Global Receive FIFO Size Register n"]
    #[inline(always)]
    pub fn usb3_grxfifosiz_iter(&self) -> impl Iterator<Item = &Usb3Grxfifosiz> {
        self.usb3_grxfifosiz.iter()
    }
    #[doc = "0xc400 - Global Event Buffer Address (Low) Register 0"]
    #[inline(always)]
    pub const fn usb3_gevntadrlo0(&self) -> &Usb3Gevntadrlo0 {
        &self.usb3_gevntadrlo0
    }
    #[doc = "0xc404 - Global Event Buffer Address (High) Register 0"]
    #[inline(always)]
    pub const fn usb3_gevntadrhi0(&self) -> &Usb3Gevntadrhi0 {
        &self.usb3_gevntadrhi0
    }
    #[doc = "0xc408 - Global Event Buffer Size Register 0"]
    #[inline(always)]
    pub const fn usb3_gevntsiz0(&self) -> &Usb3Gevntsiz0 {
        &self.usb3_gevntsiz0
    }
    #[doc = "0xc40c - Global Event Buffer Count Register 0"]
    #[inline(always)]
    pub const fn usb3_gevntcount0(&self) -> &Usb3Gevntcount0 {
        &self.usb3_gevntcount0
    }
    #[doc = "0xc600 - Global Hardware Parameters Register 8"]
    #[inline(always)]
    pub const fn usb3_ghwparams8(&self) -> &Usb3Ghwparams8 {
        &self.usb3_ghwparams8
    }
    #[doc = "0xc610 - Global Device TX FIFO DMA Priority Register"]
    #[inline(always)]
    pub const fn usb3_gtxfifopridev(&self) -> &Usb3Gtxfifopridev {
        &self.usb3_gtxfifopridev
    }
    #[doc = "0xc618 - Global Host TX FIFO DMA Priority Register"]
    #[inline(always)]
    pub const fn usb3_gtxfifoprihst(&self) -> &Usb3Gtxfifoprihst {
        &self.usb3_gtxfifoprihst
    }
    #[doc = "0xc61c - Global Host RX FIFO DMA Priority Register"]
    #[inline(always)]
    pub const fn usb3_grxfifoprihst(&self) -> &Usb3Grxfifoprihst {
        &self.usb3_grxfifoprihst
    }
    #[doc = "0xc620 - Global Host Debug Capability DMA Priority Register"]
    #[inline(always)]
    pub const fn usb3_gfifopridbc(&self) -> &Usb3Gfifopridbc {
        &self.usb3_gfifopridbc
    }
    #[doc = "0xc624 - Global Host FIFO DMA High-Low Priority Ratio Register"]
    #[inline(always)]
    pub const fn usb3_gdmahlratio(&self) -> &Usb3Gdmahlratio {
        &self.usb3_gdmahlratio
    }
    #[doc = "0xc630 - Global Frame Length Adjustment Register"]
    #[inline(always)]
    pub const fn usb3_gfladj(&self) -> &Usb3Gfladj {
        &self.usb3_gfladj
    }
    #[doc = "0xc700 - Device Configuration Register"]
    #[inline(always)]
    pub const fn usb3_dcfg(&self) -> &Usb3Dcfg {
        &self.usb3_dcfg
    }
    #[doc = "0xc704 - Device Control Register"]
    #[inline(always)]
    pub const fn usb3_dctl(&self) -> &Usb3Dctl {
        &self.usb3_dctl
    }
    #[doc = "0xc708 - Device Event Enable Register"]
    #[inline(always)]
    pub const fn usb3_devten(&self) -> &Usb3Devten {
        &self.usb3_devten
    }
    #[doc = "0xc70c - Device Status Register"]
    #[inline(always)]
    pub const fn usb3_dsts(&self) -> &Usb3Dsts {
        &self.usb3_dsts
    }
    #[doc = "0xc710 - Device Generic Command Parameter Register"]
    #[inline(always)]
    pub const fn usb3_dgcmdpar(&self) -> &Usb3Dgcmdpar {
        &self.usb3_dgcmdpar
    }
    #[doc = "0xc714 - Device Generic Command Register"]
    #[inline(always)]
    pub const fn usb3_dgcmd(&self) -> &Usb3Dgcmd {
        &self.usb3_dgcmd
    }
    #[doc = "0xc720 - Device Active USB Endpoint Enable Register"]
    #[inline(always)]
    pub const fn usb3_dalepena(&self) -> &Usb3Dalepena {
        &self.usb3_dalepena
    }
    #[doc = "0xc800..0xc834 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn usb3_depcmdpar2(&self, n: usize) -> &Usb3Depcmdpar2 {
        #[allow(clippy::no_effect)]
        [(); 13][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(51200)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc800..0xc834 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub fn usb3_depcmdpar2_iter(&self) -> impl Iterator<Item = &Usb3Depcmdpar2> {
        (0..13).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(51200)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0xc800 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn usb3_dep0cmdpar2(&self) -> &Usb3Depcmdpar2 {
        self.usb3_depcmdpar2(0)
    }
    #[doc = "0xc810 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn usb3_dep1cmdpar2(&self) -> &Usb3Depcmdpar2 {
        self.usb3_depcmdpar2(1)
    }
    #[doc = "0xc820 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn usb3_dep2cmdpar2(&self) -> &Usb3Depcmdpar2 {
        self.usb3_depcmdpar2(2)
    }
    #[doc = "0xc830 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn usb3_dep3cmdpar2(&self) -> &Usb3Depcmdpar2 {
        self.usb3_depcmdpar2(3)
    }
    #[doc = "0xc840 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn usb3_dep4cmdpar2(&self) -> &Usb3Depcmdpar2 {
        self.usb3_depcmdpar2(4)
    }
    #[doc = "0xc850 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn usb3_dep5cmdpar2(&self) -> &Usb3Depcmdpar2 {
        self.usb3_depcmdpar2(5)
    }
    #[doc = "0xc860 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn usb3_dep6cmdpar2(&self) -> &Usb3Depcmdpar2 {
        self.usb3_depcmdpar2(6)
    }
    #[doc = "0xc870 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn usb3_dep7cmdpar2(&self) -> &Usb3Depcmdpar2 {
        self.usb3_depcmdpar2(7)
    }
    #[doc = "0xc880 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn usb3_dep8cmdpar2(&self) -> &Usb3Depcmdpar2 {
        self.usb3_depcmdpar2(8)
    }
    #[doc = "0xc890 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn usb3_dep9cmdpar2(&self) -> &Usb3Depcmdpar2 {
        self.usb3_depcmdpar2(9)
    }
    #[doc = "0xc8a0 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn usb3_dep10cmdpar2(&self) -> &Usb3Depcmdpar2 {
        self.usb3_depcmdpar2(10)
    }
    #[doc = "0xc8b0 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn usb3_dep11cmdpar2(&self) -> &Usb3Depcmdpar2 {
        self.usb3_depcmdpar2(11)
    }
    #[doc = "0xc8c0 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn usb3_dep12cmdpar2(&self) -> &Usb3Depcmdpar2 {
        self.usb3_depcmdpar2(12)
    }
    #[doc = "0xc804..0xc838 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn usb3_depcmdpar1(&self, n: usize) -> &Usb3Depcmdpar1 {
        #[allow(clippy::no_effect)]
        [(); 13][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(51204)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc804..0xc838 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub fn usb3_depcmdpar1_iter(&self) -> impl Iterator<Item = &Usb3Depcmdpar1> {
        (0..13).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(51204)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0xc804 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn usb3_dep0cmdpar1(&self) -> &Usb3Depcmdpar1 {
        self.usb3_depcmdpar1(0)
    }
    #[doc = "0xc814 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn usb3_dep1cmdpar1(&self) -> &Usb3Depcmdpar1 {
        self.usb3_depcmdpar1(1)
    }
    #[doc = "0xc824 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn usb3_dep2cmdpar1(&self) -> &Usb3Depcmdpar1 {
        self.usb3_depcmdpar1(2)
    }
    #[doc = "0xc834 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn usb3_dep3cmdpar1(&self) -> &Usb3Depcmdpar1 {
        self.usb3_depcmdpar1(3)
    }
    #[doc = "0xc844 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn usb3_dep4cmdpar1(&self) -> &Usb3Depcmdpar1 {
        self.usb3_depcmdpar1(4)
    }
    #[doc = "0xc854 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn usb3_dep5cmdpar1(&self) -> &Usb3Depcmdpar1 {
        self.usb3_depcmdpar1(5)
    }
    #[doc = "0xc864 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn usb3_dep6cmdpar1(&self) -> &Usb3Depcmdpar1 {
        self.usb3_depcmdpar1(6)
    }
    #[doc = "0xc874 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn usb3_dep7cmdpar1(&self) -> &Usb3Depcmdpar1 {
        self.usb3_depcmdpar1(7)
    }
    #[doc = "0xc884 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn usb3_dep8cmdpar1(&self) -> &Usb3Depcmdpar1 {
        self.usb3_depcmdpar1(8)
    }
    #[doc = "0xc894 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn usb3_dep9cmdpar1(&self) -> &Usb3Depcmdpar1 {
        self.usb3_depcmdpar1(9)
    }
    #[doc = "0xc8a4 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn usb3_dep10cmdpar1(&self) -> &Usb3Depcmdpar1 {
        self.usb3_depcmdpar1(10)
    }
    #[doc = "0xc8b4 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn usb3_dep11cmdpar1(&self) -> &Usb3Depcmdpar1 {
        self.usb3_depcmdpar1(11)
    }
    #[doc = "0xc8c4 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn usb3_dep12cmdpar1(&self) -> &Usb3Depcmdpar1 {
        self.usb3_depcmdpar1(12)
    }
    #[doc = "0xc808..0xc83c - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn usb3_depcmdpar0(&self, n: usize) -> &Usb3Depcmdpar0 {
        #[allow(clippy::no_effect)]
        [(); 13][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(51208)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc808..0xc83c - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub fn usb3_depcmdpar0_iter(&self) -> impl Iterator<Item = &Usb3Depcmdpar0> {
        (0..13).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(51208)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0xc808 - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn usb3_dep0cmdpar0(&self) -> &Usb3Depcmdpar0 {
        self.usb3_depcmdpar0(0)
    }
    #[doc = "0xc818 - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn usb3_dep1cmdpar0(&self) -> &Usb3Depcmdpar0 {
        self.usb3_depcmdpar0(1)
    }
    #[doc = "0xc828 - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn usb3_dep2cmdpar0(&self) -> &Usb3Depcmdpar0 {
        self.usb3_depcmdpar0(2)
    }
    #[doc = "0xc838 - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn usb3_dep3cmdpar0(&self) -> &Usb3Depcmdpar0 {
        self.usb3_depcmdpar0(3)
    }
    #[doc = "0xc848 - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn usb3_dep4cmdpar0(&self) -> &Usb3Depcmdpar0 {
        self.usb3_depcmdpar0(4)
    }
    #[doc = "0xc858 - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn usb3_dep5cmdpar0(&self) -> &Usb3Depcmdpar0 {
        self.usb3_depcmdpar0(5)
    }
    #[doc = "0xc868 - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn usb3_dep6cmdpar0(&self) -> &Usb3Depcmdpar0 {
        self.usb3_depcmdpar0(6)
    }
    #[doc = "0xc878 - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn usb3_dep7cmdpar0(&self) -> &Usb3Depcmdpar0 {
        self.usb3_depcmdpar0(7)
    }
    #[doc = "0xc888 - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn usb3_dep8cmdpar0(&self) -> &Usb3Depcmdpar0 {
        self.usb3_depcmdpar0(8)
    }
    #[doc = "0xc898 - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn usb3_dep9cmdpar0(&self) -> &Usb3Depcmdpar0 {
        self.usb3_depcmdpar0(9)
    }
    #[doc = "0xc8a8 - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn usb3_dep10cmdpar0(&self) -> &Usb3Depcmdpar0 {
        self.usb3_depcmdpar0(10)
    }
    #[doc = "0xc8b8 - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn usb3_dep11cmdpar0(&self) -> &Usb3Depcmdpar0 {
        self.usb3_depcmdpar0(11)
    }
    #[doc = "0xc8c8 - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn usb3_dep12cmdpar0(&self) -> &Usb3Depcmdpar0 {
        self.usb3_depcmdpar0(12)
    }
    #[doc = "0xc80c..0xc840 - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn usb3_depcmd(&self, n: usize) -> &Usb3Depcmd {
        #[allow(clippy::no_effect)]
        [(); 13][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(51212)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc80c..0xc840 - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub fn usb3_depcmd_iter(&self) -> impl Iterator<Item = &Usb3Depcmd> {
        (0..13).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(51212)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0xc80c - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn usb3_dep0cmd(&self) -> &Usb3Depcmd {
        self.usb3_depcmd(0)
    }
    #[doc = "0xc81c - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn usb3_dep1cmd(&self) -> &Usb3Depcmd {
        self.usb3_depcmd(1)
    }
    #[doc = "0xc82c - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn usb3_dep2cmd(&self) -> &Usb3Depcmd {
        self.usb3_depcmd(2)
    }
    #[doc = "0xc83c - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn usb3_dep3cmd(&self) -> &Usb3Depcmd {
        self.usb3_depcmd(3)
    }
    #[doc = "0xc84c - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn usb3_dep4cmd(&self) -> &Usb3Depcmd {
        self.usb3_depcmd(4)
    }
    #[doc = "0xc85c - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn usb3_dep5cmd(&self) -> &Usb3Depcmd {
        self.usb3_depcmd(5)
    }
    #[doc = "0xc86c - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn usb3_dep6cmd(&self) -> &Usb3Depcmd {
        self.usb3_depcmd(6)
    }
    #[doc = "0xc87c - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn usb3_dep7cmd(&self) -> &Usb3Depcmd {
        self.usb3_depcmd(7)
    }
    #[doc = "0xc88c - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn usb3_dep8cmd(&self) -> &Usb3Depcmd {
        self.usb3_depcmd(8)
    }
    #[doc = "0xc89c - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn usb3_dep9cmd(&self) -> &Usb3Depcmd {
        self.usb3_depcmd(9)
    }
    #[doc = "0xc8ac - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn usb3_dep10cmd(&self) -> &Usb3Depcmd {
        self.usb3_depcmd(10)
    }
    #[doc = "0xc8bc - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn usb3_dep11cmd(&self) -> &Usb3Depcmd {
        self.usb3_depcmd(11)
    }
    #[doc = "0xc8cc - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn usb3_dep12cmd(&self) -> &Usb3Depcmd {
        self.usb3_depcmd(12)
    }
}
#[doc = "USB3_GSBUSCFG0 (rw) register accessor: Global SoC Bus Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gsbuscfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gsbuscfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gsbuscfg0`]
module"]
#[doc(alias = "USB3_GSBUSCFG0")]
pub type Usb3Gsbuscfg0 = crate::Reg<usb3_gsbuscfg0::Usb3Gsbuscfg0Spec>;
#[doc = "Global SoC Bus Configuration Register 0"]
pub mod usb3_gsbuscfg0;
#[doc = "USB3_GSBUSCFG1 (rw) register accessor: Global SoC Bus Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gsbuscfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gsbuscfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gsbuscfg1`]
module"]
#[doc(alias = "USB3_GSBUSCFG1")]
pub type Usb3Gsbuscfg1 = crate::Reg<usb3_gsbuscfg1::Usb3Gsbuscfg1Spec>;
#[doc = "Global SoC Bus Configuration Register 1"]
pub mod usb3_gsbuscfg1;
#[doc = "USB3_GTXTHRCFG (rw) register accessor: Global Tx Threshold Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gtxthrcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gtxthrcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gtxthrcfg`]
module"]
#[doc(alias = "USB3_GTXTHRCFG")]
pub type Usb3Gtxthrcfg = crate::Reg<usb3_gtxthrcfg::Usb3GtxthrcfgSpec>;
#[doc = "Global Tx Threshold Control Register"]
pub mod usb3_gtxthrcfg;
#[doc = "USB3_GRXTHRCFG (rw) register accessor: Global Rx Threshold Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_grxthrcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_grxthrcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_grxthrcfg`]
module"]
#[doc(alias = "USB3_GRXTHRCFG")]
pub type Usb3Grxthrcfg = crate::Reg<usb3_grxthrcfg::Usb3GrxthrcfgSpec>;
#[doc = "Global Rx Threshold Control Register"]
pub mod usb3_grxthrcfg;
#[doc = "USB3_GCTL (rw) register accessor: Global Core Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gctl`]
module"]
#[doc(alias = "USB3_GCTL")]
pub type Usb3Gctl = crate::Reg<usb3_gctl::Usb3GctlSpec>;
#[doc = "Global Core Control Register"]
pub mod usb3_gctl;
#[doc = "USB3_GPMSTS (rw) register accessor: Global Power Management Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gpmsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gpmsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gpmsts`]
module"]
#[doc(alias = "USB3_GPMSTS")]
pub type Usb3Gpmsts = crate::Reg<usb3_gpmsts::Usb3GpmstsSpec>;
#[doc = "Global Power Management Status Register"]
pub mod usb3_gpmsts;
#[doc = "USB3_GSTS (rw) register accessor: Global Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gsts`]
module"]
#[doc(alias = "USB3_GSTS")]
pub type Usb3Gsts = crate::Reg<usb3_gsts::Usb3GstsSpec>;
#[doc = "Global Status Register"]
pub mod usb3_gsts;
#[doc = "USB3_GUCTL1 (rw) register accessor: Global User Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_guctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_guctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_guctl1`]
module"]
#[doc(alias = "USB3_GUCTL1")]
pub type Usb3Guctl1 = crate::Reg<usb3_guctl1::Usb3Guctl1Spec>;
#[doc = "Global User Control Register 1"]
pub mod usb3_guctl1;
#[doc = "USB3_GSNPSID (r) register accessor: Global SNPS ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gsnpsid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gsnpsid`]
module"]
#[doc(alias = "USB3_GSNPSID")]
pub type Usb3Gsnpsid = crate::Reg<usb3_gsnpsid::Usb3GsnpsidSpec>;
#[doc = "Global SNPS ID Register"]
pub mod usb3_gsnpsid;
#[doc = "USB3_GGPIO (rw) register accessor: Global General Purpose Input/Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_ggpio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_ggpio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_ggpio`]
module"]
#[doc(alias = "USB3_GGPIO")]
pub type Usb3Ggpio = crate::Reg<usb3_ggpio::Usb3GgpioSpec>;
#[doc = "Global General Purpose Input/Output Register"]
pub mod usb3_ggpio;
#[doc = "USB3_GUID (rw) register accessor: Global User ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_guid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_guid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_guid`]
module"]
#[doc(alias = "USB3_GUID")]
pub type Usb3Guid = crate::Reg<usb3_guid::Usb3GuidSpec>;
#[doc = "Global User ID Register"]
pub mod usb3_guid;
#[doc = "USB3_GUCTL (rw) register accessor: Global User Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_guctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_guctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_guctl`]
module"]
#[doc(alias = "USB3_GUCTL")]
pub type Usb3Guctl = crate::Reg<usb3_guctl::Usb3GuctlSpec>;
#[doc = "Global User Control Register"]
pub mod usb3_guctl;
#[doc = "USB3_GBUSERRADDRLO (r) register accessor: Global SoC Bus Error Address Register - Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gbuserraddrlo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gbuserraddrlo`]
module"]
#[doc(alias = "USB3_GBUSERRADDRLO")]
pub type Usb3Gbuserraddrlo = crate::Reg<usb3_gbuserraddrlo::Usb3GbuserraddrloSpec>;
#[doc = "Global SoC Bus Error Address Register - Low"]
pub mod usb3_gbuserraddrlo;
#[doc = "USB3_GBUSERRADDRHI (r) register accessor: Global SoC Bus Error Address Register - High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gbuserraddrhi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gbuserraddrhi`]
module"]
#[doc(alias = "USB3_GBUSERRADDRHI")]
pub type Usb3Gbuserraddrhi = crate::Reg<usb3_gbuserraddrhi::Usb3GbuserraddrhiSpec>;
#[doc = "Global SoC Bus Error Address Register - High"]
pub mod usb3_gbuserraddrhi;
#[doc = "USB3_GPRTBIMAPLO (rw) register accessor: Global SS Port to Bus Instance Mapping Register - Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gprtbimaplo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gprtbimaplo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gprtbimaplo`]
module"]
#[doc(alias = "USB3_GPRTBIMAPLO")]
pub type Usb3Gprtbimaplo = crate::Reg<usb3_gprtbimaplo::Usb3GprtbimaploSpec>;
#[doc = "Global SS Port to Bus Instance Mapping Register - Low"]
pub mod usb3_gprtbimaplo;
#[doc = "USB3_GHWPARAMS0 (r) register accessor: Global Hardware Parameters Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_ghwparams0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_ghwparams0`]
module"]
#[doc(alias = "USB3_GHWPARAMS0")]
pub type Usb3Ghwparams0 = crate::Reg<usb3_ghwparams0::Usb3Ghwparams0Spec>;
#[doc = "Global Hardware Parameters Register 0"]
pub mod usb3_ghwparams0;
#[doc = "USB3_GHWPARAMS1 (r) register accessor: Global Hardware Parameters Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_ghwparams1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_ghwparams1`]
module"]
#[doc(alias = "USB3_GHWPARAMS1")]
pub type Usb3Ghwparams1 = crate::Reg<usb3_ghwparams1::Usb3Ghwparams1Spec>;
#[doc = "Global Hardware Parameters Register 1"]
pub mod usb3_ghwparams1;
#[doc = "USB3_GHWPARAMS2 (r) register accessor: Global Hardware Parameters Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_ghwparams2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_ghwparams2`]
module"]
#[doc(alias = "USB3_GHWPARAMS2")]
pub type Usb3Ghwparams2 = crate::Reg<usb3_ghwparams2::Usb3Ghwparams2Spec>;
#[doc = "Global Hardware Parameters Register 2"]
pub mod usb3_ghwparams2;
#[doc = "USB3_GHWPARAMS3 (r) register accessor: Global Hardware Parameters Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_ghwparams3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_ghwparams3`]
module"]
#[doc(alias = "USB3_GHWPARAMS3")]
pub type Usb3Ghwparams3 = crate::Reg<usb3_ghwparams3::Usb3Ghwparams3Spec>;
#[doc = "Global Hardware Parameters Register 3"]
pub mod usb3_ghwparams3;
#[doc = "USB3_GHWPARAMS4 (r) register accessor: Global Hardware Parameters Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_ghwparams4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_ghwparams4`]
module"]
#[doc(alias = "USB3_GHWPARAMS4")]
pub type Usb3Ghwparams4 = crate::Reg<usb3_ghwparams4::Usb3Ghwparams4Spec>;
#[doc = "Global Hardware Parameters Register 4"]
pub mod usb3_ghwparams4;
#[doc = "USB3_GHWPARAMS5 (r) register accessor: Global Hardware Parameters Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_ghwparams5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_ghwparams5`]
module"]
#[doc(alias = "USB3_GHWPARAMS5")]
pub type Usb3Ghwparams5 = crate::Reg<usb3_ghwparams5::Usb3Ghwparams5Spec>;
#[doc = "Global Hardware Parameters Register 5"]
pub mod usb3_ghwparams5;
#[doc = "USB3_GHWPARAMS6 (r) register accessor: Global Hardware Parameters Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_ghwparams6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_ghwparams6`]
module"]
#[doc(alias = "USB3_GHWPARAMS6")]
pub type Usb3Ghwparams6 = crate::Reg<usb3_ghwparams6::Usb3Ghwparams6Spec>;
#[doc = "Global Hardware Parameters Register 6"]
pub mod usb3_ghwparams6;
#[doc = "USB3_GHWPARAMS7 (r) register accessor: Global Hardware Parameters Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_ghwparams7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_ghwparams7`]
module"]
#[doc(alias = "USB3_GHWPARAMS7")]
pub type Usb3Ghwparams7 = crate::Reg<usb3_ghwparams7::Usb3Ghwparams7Spec>;
#[doc = "Global Hardware Parameters Register 7"]
pub mod usb3_ghwparams7;
#[doc = "USB3_GDBGFIFOSPACE (rw) register accessor: Global Debug Queue/FIFO Space Available Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gdbgfifospace::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gdbgfifospace::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gdbgfifospace`]
module"]
#[doc(alias = "USB3_GDBGFIFOSPACE")]
pub type Usb3Gdbgfifospace = crate::Reg<usb3_gdbgfifospace::Usb3GdbgfifospaceSpec>;
#[doc = "Global Debug Queue/FIFO Space Available Register"]
pub mod usb3_gdbgfifospace;
#[doc = "USB3_GDBGLTSSM (rw) register accessor: Global Debug LTSSM Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gdbgltssm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gdbgltssm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gdbgltssm`]
module"]
#[doc(alias = "USB3_GDBGLTSSM")]
pub type Usb3Gdbgltssm = crate::Reg<usb3_gdbgltssm::Usb3GdbgltssmSpec>;
#[doc = "Global Debug LTSSM Register"]
pub mod usb3_gdbgltssm;
#[doc = "USB3_GDBGLNMCC (r) register accessor: Global Debug LNMCC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gdbglnmcc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gdbglnmcc`]
module"]
#[doc(alias = "USB3_GDBGLNMCC")]
pub type Usb3Gdbglnmcc = crate::Reg<usb3_gdbglnmcc::Usb3GdbglnmccSpec>;
#[doc = "Global Debug LNMCC Register"]
pub mod usb3_gdbglnmcc;
#[doc = "USB3_GDBGBMU (rw) register accessor: Global Debug BMU Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gdbgbmu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gdbgbmu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gdbgbmu`]
module"]
#[doc(alias = "USB3_GDBGBMU")]
pub type Usb3Gdbgbmu = crate::Reg<usb3_gdbgbmu::Usb3GdbgbmuSpec>;
#[doc = "Global Debug BMU Register"]
pub mod usb3_gdbgbmu;
#[doc = "USB3_GDBGLSPMUX (rw) register accessor: Global Debug LSP MUX Register - Device\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gdbglspmux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gdbglspmux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gdbglspmux`]
module"]
#[doc(alias = "USB3_GDBGLSPMUX")]
pub type Usb3Gdbglspmux = crate::Reg<usb3_gdbglspmux::Usb3GdbglspmuxSpec>;
#[doc = "Global Debug LSP MUX Register - Device"]
pub mod usb3_gdbglspmux;
#[doc = "USB3_GDBGLSP (r) register accessor: Global Debug LSP Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gdbglsp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gdbglsp`]
module"]
#[doc(alias = "USB3_GDBGLSP")]
pub type Usb3Gdbglsp = crate::Reg<usb3_gdbglsp::Usb3GdbglspSpec>;
#[doc = "Global Debug LSP Register"]
pub mod usb3_gdbglsp;
#[doc = "USB3_GDBGEPINFO0 (r) register accessor: Global Debug Endpoint Information Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gdbgepinfo0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gdbgepinfo0`]
module"]
#[doc(alias = "USB3_GDBGEPINFO0")]
pub type Usb3Gdbgepinfo0 = crate::Reg<usb3_gdbgepinfo0::Usb3Gdbgepinfo0Spec>;
#[doc = "Global Debug Endpoint Information Register 0"]
pub mod usb3_gdbgepinfo0;
#[doc = "USB3_GDBGEPINFO1 (r) register accessor: Global Debug Endpoint Information Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gdbgepinfo1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gdbgepinfo1`]
module"]
#[doc(alias = "USB3_GDBGEPINFO1")]
pub type Usb3Gdbgepinfo1 = crate::Reg<usb3_gdbgepinfo1::Usb3Gdbgepinfo1Spec>;
#[doc = "Global Debug Endpoint Information Register 1"]
pub mod usb3_gdbgepinfo1;
#[doc = "USB3_GPRTBIMAP_HSLO (rw) register accessor: Global High-Speed Port to Bus Instance Mapping Register - Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gprtbimap_hslo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gprtbimap_hslo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gprtbimap_hslo`]
module"]
#[doc(alias = "USB3_GPRTBIMAP_HSLO")]
pub type Usb3GprtbimapHslo = crate::Reg<usb3_gprtbimap_hslo::Usb3GprtbimapHsloSpec>;
#[doc = "Global High-Speed Port to Bus Instance Mapping Register - Low"]
pub mod usb3_gprtbimap_hslo;
#[doc = "USB3_GPRTBIMAP_FSLO (rw) register accessor: Global Full-Speed Port to Bus Instance Mapping Register - Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gprtbimap_fslo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gprtbimap_fslo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gprtbimap_fslo`]
module"]
#[doc(alias = "USB3_GPRTBIMAP_FSLO")]
pub type Usb3GprtbimapFslo = crate::Reg<usb3_gprtbimap_fslo::Usb3GprtbimapFsloSpec>;
#[doc = "Global Full-Speed Port to Bus Instance Mapping Register - Low"]
pub mod usb3_gprtbimap_fslo;
#[doc = "USB3_GUSB2PHYCFG0 (rw) register accessor: Global USB2 PHY Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gusb2phycfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gusb2phycfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gusb2phycfg0`]
module"]
#[doc(alias = "USB3_GUSB2PHYCFG0")]
pub type Usb3Gusb2phycfg0 = crate::Reg<usb3_gusb2phycfg0::Usb3Gusb2phycfg0Spec>;
#[doc = "Global USB2 PHY Configuration Register 0"]
pub mod usb3_gusb2phycfg0;
#[doc = "USB3_GUSB3PIPECTL0 (rw) register accessor: Global USB3 PIPE Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gusb3pipectl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gusb3pipectl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gusb3pipectl0`]
module"]
#[doc(alias = "USB3_GUSB3PIPECTL0")]
pub type Usb3Gusb3pipectl0 = crate::Reg<usb3_gusb3pipectl0::Usb3Gusb3pipectl0Spec>;
#[doc = "Global USB3 PIPE Control Register 0"]
pub mod usb3_gusb3pipectl0;
#[doc = "USB3_GTXFIFOSIZ (rw) register accessor: Global Transmit FIFO Size Register n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gtxfifosiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gtxfifosiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gtxfifosiz`]
module"]
#[doc(alias = "USB3_GTXFIFOSIZ")]
pub type Usb3Gtxfifosiz = crate::Reg<usb3_gtxfifosiz::Usb3GtxfifosizSpec>;
#[doc = "Global Transmit FIFO Size Register n"]
pub mod usb3_gtxfifosiz;
#[doc = "USB3_GRXFIFOSIZ (rw) register accessor: Global Receive FIFO Size Register n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_grxfifosiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_grxfifosiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_grxfifosiz`]
module"]
#[doc(alias = "USB3_GRXFIFOSIZ")]
pub type Usb3Grxfifosiz = crate::Reg<usb3_grxfifosiz::Usb3GrxfifosizSpec>;
#[doc = "Global Receive FIFO Size Register n"]
pub mod usb3_grxfifosiz;
#[doc = "USB3_GEVNTADRLO0 (rw) register accessor: Global Event Buffer Address (Low) Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gevntadrlo0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gevntadrlo0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gevntadrlo0`]
module"]
#[doc(alias = "USB3_GEVNTADRLO0")]
pub type Usb3Gevntadrlo0 = crate::Reg<usb3_gevntadrlo0::Usb3Gevntadrlo0Spec>;
#[doc = "Global Event Buffer Address (Low) Register 0"]
pub mod usb3_gevntadrlo0;
#[doc = "USB3_GEVNTADRHI0 (rw) register accessor: Global Event Buffer Address (High) Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gevntadrhi0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gevntadrhi0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gevntadrhi0`]
module"]
#[doc(alias = "USB3_GEVNTADRHI0")]
pub type Usb3Gevntadrhi0 = crate::Reg<usb3_gevntadrhi0::Usb3Gevntadrhi0Spec>;
#[doc = "Global Event Buffer Address (High) Register 0"]
pub mod usb3_gevntadrhi0;
#[doc = "USB3_GEVNTSIZ0 (rw) register accessor: Global Event Buffer Size Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gevntsiz0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gevntsiz0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gevntsiz0`]
module"]
#[doc(alias = "USB3_GEVNTSIZ0")]
pub type Usb3Gevntsiz0 = crate::Reg<usb3_gevntsiz0::Usb3Gevntsiz0Spec>;
#[doc = "Global Event Buffer Size Register 0"]
pub mod usb3_gevntsiz0;
#[doc = "USB3_GEVNTCOUNT0 (rw) register accessor: Global Event Buffer Count Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gevntcount0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gevntcount0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gevntcount0`]
module"]
#[doc(alias = "USB3_GEVNTCOUNT0")]
pub type Usb3Gevntcount0 = crate::Reg<usb3_gevntcount0::Usb3Gevntcount0Spec>;
#[doc = "Global Event Buffer Count Register 0"]
pub mod usb3_gevntcount0;
#[doc = "USB3_GHWPARAMS8 (r) register accessor: Global Hardware Parameters Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_ghwparams8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_ghwparams8`]
module"]
#[doc(alias = "USB3_GHWPARAMS8")]
pub type Usb3Ghwparams8 = crate::Reg<usb3_ghwparams8::Usb3Ghwparams8Spec>;
#[doc = "Global Hardware Parameters Register 8"]
pub mod usb3_ghwparams8;
#[doc = "USB3_GTXFIFOPRIDEV (rw) register accessor: Global Device TX FIFO DMA Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gtxfifopridev::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gtxfifopridev::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gtxfifopridev`]
module"]
#[doc(alias = "USB3_GTXFIFOPRIDEV")]
pub type Usb3Gtxfifopridev = crate::Reg<usb3_gtxfifopridev::Usb3GtxfifopridevSpec>;
#[doc = "Global Device TX FIFO DMA Priority Register"]
pub mod usb3_gtxfifopridev;
#[doc = "USB3_GTXFIFOPRIHST (rw) register accessor: Global Host TX FIFO DMA Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gtxfifoprihst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gtxfifoprihst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gtxfifoprihst`]
module"]
#[doc(alias = "USB3_GTXFIFOPRIHST")]
pub type Usb3Gtxfifoprihst = crate::Reg<usb3_gtxfifoprihst::Usb3GtxfifoprihstSpec>;
#[doc = "Global Host TX FIFO DMA Priority Register"]
pub mod usb3_gtxfifoprihst;
#[doc = "USB3_GRXFIFOPRIHST (rw) register accessor: Global Host RX FIFO DMA Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_grxfifoprihst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_grxfifoprihst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_grxfifoprihst`]
module"]
#[doc(alias = "USB3_GRXFIFOPRIHST")]
pub type Usb3Grxfifoprihst = crate::Reg<usb3_grxfifoprihst::Usb3GrxfifoprihstSpec>;
#[doc = "Global Host RX FIFO DMA Priority Register"]
pub mod usb3_grxfifoprihst;
#[doc = "USB3_GFIFOPRIDBC (rw) register accessor: Global Host Debug Capability DMA Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gfifopridbc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gfifopridbc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gfifopridbc`]
module"]
#[doc(alias = "USB3_GFIFOPRIDBC")]
pub type Usb3Gfifopridbc = crate::Reg<usb3_gfifopridbc::Usb3GfifopridbcSpec>;
#[doc = "Global Host Debug Capability DMA Priority Register"]
pub mod usb3_gfifopridbc;
#[doc = "USB3_GDMAHLRATIO (rw) register accessor: Global Host FIFO DMA High-Low Priority Ratio Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gdmahlratio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gdmahlratio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gdmahlratio`]
module"]
#[doc(alias = "USB3_GDMAHLRATIO")]
pub type Usb3Gdmahlratio = crate::Reg<usb3_gdmahlratio::Usb3GdmahlratioSpec>;
#[doc = "Global Host FIFO DMA High-Low Priority Ratio Register"]
pub mod usb3_gdmahlratio;
#[doc = "USB3_GFLADJ (rw) register accessor: Global Frame Length Adjustment Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gfladj::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gfladj::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_gfladj`]
module"]
#[doc(alias = "USB3_GFLADJ")]
pub type Usb3Gfladj = crate::Reg<usb3_gfladj::Usb3GfladjSpec>;
#[doc = "Global Frame Length Adjustment Register"]
pub mod usb3_gfladj;
#[doc = "USB3_DCFG (rw) register accessor: Device Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_dcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_dcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_dcfg`]
module"]
#[doc(alias = "USB3_DCFG")]
pub type Usb3Dcfg = crate::Reg<usb3_dcfg::Usb3DcfgSpec>;
#[doc = "Device Configuration Register"]
pub mod usb3_dcfg;
#[doc = "USB3_DCTL (rw) register accessor: Device Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_dctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_dctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_dctl`]
module"]
#[doc(alias = "USB3_DCTL")]
pub type Usb3Dctl = crate::Reg<usb3_dctl::Usb3DctlSpec>;
#[doc = "Device Control Register"]
pub mod usb3_dctl;
#[doc = "USB3_DEVTEN (rw) register accessor: Device Event Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_devten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_devten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_devten`]
module"]
#[doc(alias = "USB3_DEVTEN")]
pub type Usb3Devten = crate::Reg<usb3_devten::Usb3DevtenSpec>;
#[doc = "Device Event Enable Register"]
pub mod usb3_devten;
#[doc = "USB3_DSTS (r) register accessor: Device Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_dsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_dsts`]
module"]
#[doc(alias = "USB3_DSTS")]
pub type Usb3Dsts = crate::Reg<usb3_dsts::Usb3DstsSpec>;
#[doc = "Device Status Register"]
pub mod usb3_dsts;
#[doc = "USB3_DGCMDPAR (rw) register accessor: Device Generic Command Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_dgcmdpar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_dgcmdpar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_dgcmdpar`]
module"]
#[doc(alias = "USB3_DGCMDPAR")]
pub type Usb3Dgcmdpar = crate::Reg<usb3_dgcmdpar::Usb3DgcmdparSpec>;
#[doc = "Device Generic Command Parameter Register"]
pub mod usb3_dgcmdpar;
#[doc = "USB3_DGCMD (rw) register accessor: Device Generic Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_dgcmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_dgcmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_dgcmd`]
module"]
#[doc(alias = "USB3_DGCMD")]
pub type Usb3Dgcmd = crate::Reg<usb3_dgcmd::Usb3DgcmdSpec>;
#[doc = "Device Generic Command Register"]
pub mod usb3_dgcmd;
#[doc = "USB3_DALEPENA (rw) register accessor: Device Active USB Endpoint Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_dalepena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_dalepena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_dalepena`]
module"]
#[doc(alias = "USB3_DALEPENA")]
pub type Usb3Dalepena = crate::Reg<usb3_dalepena::Usb3DalepenaSpec>;
#[doc = "Device Active USB Endpoint Enable Register"]
pub mod usb3_dalepena;
#[doc = "USB3_DEPCMDPAR2 (rw) register accessor: Device Physical Endpoint-n Command Parameter 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_depcmdpar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_depcmdpar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_depcmdpar2`]
module"]
#[doc(alias = "USB3_DEPCMDPAR2")]
pub type Usb3Depcmdpar2 = crate::Reg<usb3_depcmdpar2::Usb3Depcmdpar2Spec>;
#[doc = "Device Physical Endpoint-n Command Parameter 2 Register"]
pub mod usb3_depcmdpar2;
#[doc = "USB3_DEPCMDPAR1 (rw) register accessor: Device Physical Endpoint-n Command Parameter 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_depcmdpar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_depcmdpar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_depcmdpar1`]
module"]
#[doc(alias = "USB3_DEPCMDPAR1")]
pub type Usb3Depcmdpar1 = crate::Reg<usb3_depcmdpar1::Usb3Depcmdpar1Spec>;
#[doc = "Device Physical Endpoint-n Command Parameter 1 Register"]
pub mod usb3_depcmdpar1;
#[doc = "USB3_DEPCMDPAR0 (rw) register accessor: Device Physical Endpoint-n Command Parameter 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_depcmdpar0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_depcmdpar0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_depcmdpar0`]
module"]
#[doc(alias = "USB3_DEPCMDPAR0")]
pub type Usb3Depcmdpar0 = crate::Reg<usb3_depcmdpar0::Usb3Depcmdpar0Spec>;
#[doc = "Device Physical Endpoint-n Command Parameter 0 Register"]
pub mod usb3_depcmdpar0;
#[doc = "USB3_DEPCMD (rw) register accessor: Device Physical Endpoint-n Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_depcmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_depcmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_depcmd`]
module"]
#[doc(alias = "USB3_DEPCMD")]
pub type Usb3Depcmd = crate::Reg<usb3_depcmd::Usb3DepcmdSpec>;
#[doc = "Device Physical Endpoint-n Command Register"]
pub mod usb3_depcmd;
